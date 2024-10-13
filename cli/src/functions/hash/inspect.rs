use crate::{
    elements::FuncArg,
    error::LinkedErr,
    functions::{ExecutorPinnedResult, E},
    inf::{tools::get_name, Context, Scope, Value},
};
use blake3::Hasher;
use fshasher::{
    hasher::blake::Blake, reader::buffering::Buffering, Entry, Filter, Options as HasherOptions,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

impl From<bstorage::E> for E {
    fn from(err: bstorage::E) -> Self {
        E::Other(format!("Storage error: {err}"))
    }
}

impl From<fshasher::E> for E {
    fn from(err: fshasher::E) -> Self {
        E::Other(format!("FSHasher error: {err}"))
    }
}

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

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(
    args: Vec<FuncArg>,
    args_token: usize,
    cx: Context,
    sc: Scope,
) -> ExecutorPinnedResult {
    Box::pin(async move {
        if args.len() != 3 {
            return Err(LinkedErr::new(E::InvalidFunctionArg(
                if args.len() == 1 {
                    "Missed paths to exclude from inspecting: hash::inspect(string[], string[] <= missed argument, bool)"
                } else if args.len() == 2 {
                    "Missed flag to consider or not .gitignore: hash::inspect(string[], string[], bool <= missed argument)"
                } else {
                    "No arguments for hash::inspect(string[], string[], bool)"
                }.to_string(),
            ), Some(args_token)))?;
        }
        let cwd = sc.get_cwd().await?;
        let dests = if let Some(patterns) = args[0].value.as_path_bufs() {
            patterns.into_iter().map(|p| cwd.join(p)).collect()
        } else if let Some(pattern) = args[0].value.as_path_buf() {
            vec![cwd.join(pattern)]
        } else {
            return Err(args[0].err(E::InvalidFunctionArg(
                "Invalid argument; expecting string[] | string".to_string(),
            )));
        };
        let mut exclude = if let Some(patterns) = args[1].value.as_strings() {
            patterns
        } else if let Some(pattern) = args[1].value.as_string() {
            vec![pattern]
        } else {
            return Err(args[1].err(E::InvalidFunctionArg(
                "Invalid argument; expecting string[] | string".to_string(),
            )));
        };
        let Some(gitignore) = args[2].value.as_bool() else {
            return Err(args[2].err(E::InvalidFunctionArg(
                "Invalid argument; expecting bool".to_string(),
            )));
        };
        let mut storage = cx.get_storage()?;
        let mut summary = Vec::new();
        dests.iter().for_each(|p| {
            if !p.exists() {
                cx.journal
                    .warn("hash::inspect", format!("{p:?}: doesn't exist"));
            }
        });
        exclude.push("**/.sibs".to_string());
        for path in dests.iter() {
            let mut entry = Entry::from(path)
                .map_err(|e| E::Other(format!("Cannot set entry for fshasher: {e}")))?;
            for rule in exclude.iter() {
                entry = entry
                    .exclude(Filter::Common(rule))
                    .map_err(|e| E::Other(format!("Cannot set entry for fshasher: {e}")))?;
            }
            if gitignore {
                entry = entry.context(fshasher::ContextFile::Ignore(".gitignore"));
            }
            let hash = HasherOptions::new()
                .entry(entry)?
                .tolerance(fshasher::Tolerance::LogErrors)
                .walker()?
                .collect()?
                .hash::<Blake, Buffering>()?
                .to_vec();
            let key = as_hash(path);
            let prev = storage.get_or_default::<PrevHash, &str>(&key)?;
            let same = hash == prev.hash;
            storage.set(&key, &PrevHash { hash })?;
            summary.push(same);
        }
        Ok(Value::bool(summary.iter().any(|same| !same)))
    })
}

#[cfg(test)]
mod test {
    use crate::{
        elements::{ElementId, Element},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            tests::*,
            Configuration, Context, ExecuteContext, Journal, Scope, Value,
        },
        process_string,
        reader::{Reader, Sources},
    };

    type CaseExpectation<'a> = (&'a [&'a str], bool, Value);
    const CASES: &[&[CaseExpectation]] = &[
        &[
            (&["test_a", "a"], false, Value::bool(true)),
            (&["test_a", "a"], false, Value::Empty(())),
            (&["test_a", "a"], true, Value::bool(true)),
            (&["test_a", "b"], false, Value::bool(true)),
            (&["test_a", "b"], false, Value::bool(true)),
        ],
        &[
            (&["test_b", "a"], true, Value::bool(true)),
            (&["test_b", "a"], false, Value::Empty(())),
            (&["test_b", "b"], false, Value::Empty(())),
            (&["test_b", "a"], true, Value::bool(true)),
            (&["test_b", "b"], true, Value::bool(true)),
        ],
        &[
            (&["test_c", "a"], true, Value::bool(true)),
            (&["test_c", "a"], false, Value::Empty(())),
            (&["test_c", "b"], false, Value::Empty(())),
            (&["test_c", "a"], true, Value::bool(true)),
            (&["test_c", "b"], true, Value::bool(true)),
        ],
    ];
    const TEST: &str = r#"
#(a: ./)
    hash::inspect((__paths__), (), true) -> (:test_a("a"));
    @test_a($a: a | b) {
        print("Task A test is done with {$a}");
        true;
    };
#(b: ./)
    hash::inspect((__paths__), (), true) -> (:test_b("a"), :test_b("b"));
    @test_b($a: a | b) {
        print("Task B test is done with {$a}");
        true;
    };
#(c: ./)
    hash::inspect((__paths__), (), true) -> ();
    @test_c($a: a | b) {
        print("Task C test is done with {$a}");
        true;
    };"#;
    const PATHS_HOOK: &str = "__paths__";

    #[tokio::test]
    async fn processing() {
        let mut usecases = [
            UseCase::gen(Strategy::Number(3), Strategy::Number(10), 3).expect("Usecase is created"),
            UseCase::gen(Strategy::Number(3), Strategy::Number(10), 3).expect("Usecase is created"),
        ];
        let content = TEST.replace(
            PATHS_HOOK,
            &usecases
                .iter()
                .map(|uc| format!("\"{}\"", uc.root.display()))
                .collect::<Vec<String>>()
                .join(","),
        );
        process_string!(
            &Configuration::logs(false),
            &content,
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElementId::Component]))?
                {
                    components.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(components)
            },
            |components: Vec<Element>, cx: Context, sc: Scope, _journal: Journal| async move {
                for (n, component) in components.iter().enumerate() {
                    let case = CASES[n];
                    for (args, needs_changes, expected_result) in case {
                        if *needs_changes {
                            for usecase in usecases.iter_mut() {
                                usecase.change(1).expect("UseCase has been changed");
                            }
                        }
                        let result = component
                            .execute(
                                ExecuteContext::unbound(cx.clone(), sc.clone())
                                    .owner(Some(component))
                                    .components(&components)
                                    .args(
                                        &args
                                            .iter()
                                            .map(|s| Value::String(s.to_string()))
                                            .collect::<Vec<Value>>(),
                                    ),
                            )
                            .await?;
                        assert_eq!(result, *expected_result);
                    }
                }
                for usecase in usecases.iter_mut() {
                    usecase.clean().expect("UseCase has been changed");
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
