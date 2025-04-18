use crate::*;

declare_embedded_fn!(
    vec![
        Ty::OneOf(vec![
            DeterminedTy::PathBuf,
            DeterminedTy::Str,
            DeterminedTy::Vec(Some(Box::new(DeterminedTy::Str))),
            DeterminedTy::Vec(Some(Box::new(DeterminedTy::PathBuf))),
            DeterminedTy::Vec(Some(Box::new(DeterminedTy::Void))),
        ]),
        Ty::OneOf(vec![
            DeterminedTy::Str,
            DeterminedTy::Vec(Some(Box::new(DeterminedTy::Str))),
            DeterminedTy::Vec(Some(Box::new(DeterminedTy::Void))),
        ]),
        Ty::Determined(DeterminedTy::Bool),
    ],
    DeterminedTy::Bool
);

use blake3::Hasher;
use fshasher::{
    hasher::blake::Blake, reader::buffering::Buffering, Entry, Filter, Options as HasherOptions,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

fn as_hash<P: AsRef<Path>>(path: P) -> String {
    Hasher::new()
        .update(path.as_ref().to_string_lossy().as_bytes())
        .finalize()
        .to_hex()
        .to_lowercase()
        .to_string()
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct PrevHash {
    hash: Vec<u8>,
}

#[boxed]
pub fn executor(
    args: Vec<FnArgValue>,
    _rt: Runtime,
    cx: Context,
    caller: SrcLink,
) -> RtPinnedResult<'static, LinkedErr<E>> {
    if args.len() != 3 {
        return Err(LinkedErr::by_link(
            E::InvalidFnArgumentsNumber(3, args.len()),
            (&caller).into(),
        ));
    }
    let root = cx
        .cwd()
        .root()
        .await
        .map_err(|err| LinkedErr::by_link(err, (&caller).into()))?;
    // Get list of paths to check
    let arg = &args[0];
    let paths = match &arg.value {
        RtValue::PathBuf(path) => {
            vec![root.join(path)]
        }
        RtValue::Str(path) => {
            vec![root.join(PathBuf::from(path))]
        }
        RtValue::Vec(paths) => {
            let mut processed = Vec::new();
            for path in paths.iter() {
                match path {
                    RtValue::PathBuf(path) => {
                        processed.push(root.join(path));
                    }
                    RtValue::Str(path) => {
                        processed.push(root.join(PathBuf::from(path)));
                    }
                    _ => {
                        return Err(LinkedErr::by_link(
                            E::DismatchValueType(
                                RtValueId::Str.to_string(),
                                arg.value.id().to_string(),
                            ),
                            (&arg.link).into(),
                        ));
                    }
                }
            }
            processed
        }
        _ => {
            return Err(LinkedErr::by_link(
                E::DismatchValueType(RtValueId::Str.to_string(), arg.value.id().to_string()),
                (&arg.link).into(),
            ));
        }
    };
    // Get expections
    let arg = &args[1];
    let mut expections = match &arg.value {
        RtValue::Str(exp) => {
            vec![exp.to_owned()]
        }
        RtValue::Vec(exps) => {
            let mut processed = Vec::new();
            for exp in exps.iter() {
                match exp {
                    RtValue::Str(exp) => {
                        processed.push(exp.to_owned());
                    }
                    _ => {
                        return Err(LinkedErr::by_link(
                            E::DismatchValueType(
                                RtValueId::Str.to_string(),
                                arg.value.id().to_string(),
                            ),
                            (&arg.link).into(),
                        ));
                    }
                }
            }
            processed
        }
        _ => {
            return Err(LinkedErr::by_link(
                E::DismatchValueType(RtValueId::Str.to_string(), arg.value.id().to_string()),
                (&arg.link).into(),
            ));
        }
    };
    expections = expections
        .into_iter()
        .filter_map(|v| {
            if v.trim().is_empty() {
                None
            } else {
                Some(v.trim().to_owned())
            }
        })
        .collect();
    // Consider or not .gitignore file
    let RtValue::Bool(gitignore) = args[2].value.clone() else {
        return Err(LinkedErr::by_link(
            E::DismatchValueType(RtValueId::Bool.to_string(), args[2].value.id().to_string()),
            (&args[2].link).into(),
        ));
    };
    let not_exist = paths
        .iter()
        .filter(|p| !p.exists())
        .collect::<Vec<&PathBuf>>();
    if !not_exist.is_empty() {
        not_exist.iter().for_each(|p| {
            cx.job
                .journal
                .warn(format!("{}: doesn't exist", p.to_string_lossy()));
        });
        cx.job
        .journal
        .warn("Hasher will not procceed oparation and returns false-state because there are not exist paths");
        return Ok(RtValue::Bool(false));
    }
    // Get storage
    let mut storage = cx
        .storage()
        .await
        .map_err(|err| LinkedErr::by_link(err, (&caller).into()))?;
    let mut summary = Vec::new();
    let paths = paths
        .into_iter()
        .map(|p| p.canonicalize())
        .collect::<Result<Vec<PathBuf>, _>>()
        .map_err(|err| LinkedErr::by_link(err.into(), (&caller).into()))?;
    paths.iter().for_each(|p| {
        cx.job.journal.info(format!(
            "{}: will be inspected by hasher",
            p.to_string_lossy()
        ));
    });
    expections.push("**/.sibs".to_string());
    for path in paths.iter() {
        let mut entry = Entry::from(path)
            .map_err(|err| LinkedErr::by_link(E::Other(err.to_string()), (&caller).into()))?;
        for rule in expections.iter() {
            entry = entry
                .exclude(Filter::Common(rule))
                .map_err(|err| LinkedErr::by_link(E::Other(err.to_string()), (&caller).into()))?;
        }
        if gitignore {
            entry = entry.context(fshasher::ContextFile::Ignore(".gitignore"));
        }
        let hash = HasherOptions::new()
            .entry(entry)
            .map_err(|err| linked(&caller, err))?
            .tolerance(fshasher::Tolerance::LogErrors)
            .walker()
            .map_err(|err| linked(&caller, err))?
            .collect()
            .map_err(|err| linked(&caller, err))?
            .hash::<Blake, Buffering>()
            .map_err(|err| linked(&caller, err))?
            .to_vec();
        let key = as_hash(path);
        let prev = storage
            .get_or_default::<PrevHash, &str>(&key)
            .map_err(|err| LinkedErr::by_link(err.into(), (&caller).into()))?;
        let same = hash == prev.hash;
        storage
            .set(&key, &PrevHash { hash })
            .map_err(|err| LinkedErr::by_link(err.into(), (&caller).into()))?;
        summary.push(same);
    }
    Ok(RtValue::Bool(!summary.iter().any(|same| !same)))
}

fn linked(link: &SrcLink, err: fshasher::E) -> LinkedErr<E> {
    LinkedErr::by_link(E::Other(err.to_string()), link.into())
}
