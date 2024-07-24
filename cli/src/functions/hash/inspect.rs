use std::path::Path;

use crate::{
    functions::{ExecutorPinnedResult, E},
    inf::{tools::get_name, AnyValue, Context, Scope},
};
use blake3::Hasher;
use bstorage::Storage;
use fshasher::{
    hasher::blake::Blake, reader::buffering::Buffering, Entry, Filter, Options as HasherOptions,
};
use serde::{Deserialize, Serialize};

/// TODO:
/// - PatternFilter isn't available
/// - Check why exclude doesn't work with Filter
///
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

pub fn execute(args: Vec<AnyValue>, cx: Context, sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move {
        if args.len() != 3 {
            return Err(E::InvalidFunctionArg(
                if args.len() == 1 {
                    "Missed paths to exclude from inspecting: @hash::inspect(string[], string[] <= missed argument, bool)"
                } else if args.len() == 2 {
                    "Missed flag to consider or not .gitignore: @hash::inspect(string[], string[], bool <= missed argument)"
                } else {
                    "No arguments for @hash::inspect(string[], string[], bool)"
                }.to_string(),
            ));
        }
        let cwd = sc
            .get_cwd()
            .await?
            .ok_or(E::IO(String::from("No CWD path")))?;
        let dests = if let Some(patterns) = args[0].as_path_bufs() {
            patterns.into_iter().map(|p| cwd.join(p)).collect()
        } else if let Some(pattern) = args[0].as_path_buf() {
            vec![cwd.join(pattern)]
        } else {
            return Err(E::InvalidFunctionArg(
                "Invalid argument; expecting string[] | string".to_string(),
            ));
        };
        let mut exclude = if let Some(patterns) = args[1].as_strings() {
            patterns
        } else if let Some(pattern) = args[1].as_string() {
            vec![pattern]
        } else {
            return Err(E::InvalidFunctionArg(
                "Invalid argument; expecting string[] | string".to_string(),
            ));
        };
        let Some(gitignore) = args[2].as_bool() else {
            return Err(E::InvalidFunctionArg(
                "Invalid argument; expecting bool".to_string(),
            ));
        };
        let mut storage = Storage::create(cwd.join(".sibs"))?;
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
        Ok(AnyValue::bool(summary.iter().any(|same| !same)))
    })
}

#[cfg(test)]
mod test {
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::Component,
        error::LinkedErr,
        inf::{
            operator::{Operator, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Reading, Sources},
    };

    const CASES: &[&[(&[&str], bool)]] = &[
        &[
            (&["test_a", "a"], true),
            // (&["test_a", "a"], false),
            // (&["test_a", "b"], true),
        ],
        &[
            (&["test_b", "a"], true),
            // (&["test_b", "a"], false),
            // (&["test_b", "b"], false),
        ],
        &[
            (&["test_c", "a"], true),
            // (&["test_c", "a"], false),
            // (&["test_c", "b"], false),
        ],
    ];
    const TEST_ERR: &str = r#"
#(a: ./)
    @hash::inspect(("./src/"; "../cli"); (); true) -> (:wrong_name("a"));
    test_a($a: a | b) [
        @print("[A] Task test is done with {$a}");
    ];
#(b: ./)
    @hash::inspect(("./src/"; "../cli"); (); true) -> (:test_b("a"); :wrong_name("b"));
    test_b($a: a | b) [
        @print("[B] Task test is done with {$a}");
    ];"#;
    const TEST: &str = r#"
#(a: ./)
    @hash::inspect(("./src/"; "../cli"); (); true) -> (:test_a("a"));
    test_a($a: a | b) [
        @print("Task A test is done with {$a}");
    ];
#(b: ./)
    @hash::inspect(("./src/"; "../cli"); (); true) -> (:test_b("a"); :test_b("b"));
    test_b($a: a | b) [
        @print("Task B test is done with {$a}");
    ];
#(c: ./)
    @hash::inspect(("./src/"; "../cli"); (); true) -> ();
    test_c($a: a | b) [
        @print("Task C test is done with {$a}");
    ];"#;

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &TEST,
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Component> = Vec::new();
                while let Some(component) = src.report_err_if(Component::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    components.push(component);
                }
                Ok::<Vec<Component>, LinkedErr<E>>(components)
            },
            |components: Vec<Component>, cx: Context, sc: Scope, journal: Journal| async move {
                for (n, component) in components.iter().enumerate() {
                    let case = CASES[n];
                    for (args, res) in case {
                        let result = component
                            .execute(
                                Some(component),
                                &components,
                                &args.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
                                cx.clone(),
                                sc.clone(),
                                CancellationToken::new(),
                            )
                            .await?;
                        println!("{result:?}");
                    }
                }
                // for task in tasks.iter() {
                //     let result = task
                //         .execute(
                //             None,
                //             &[],
                //             &[],
                //             cx.clone(),
                //             sc.clone(),
                //             CancellationToken::new(),
                //         )
                //         .await;
                //     if let Err(err) = result.as_ref() {
                //         journal.report(err.into());
                //     }
                //     assert_eq!(
                //         result
                //             .expect("run of task is success")
                //             .expect("test returns some value")
                //             .as_string()
                //             .expect("test returns string value"),
                //         "true".to_owned()
                //     );
                // }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
