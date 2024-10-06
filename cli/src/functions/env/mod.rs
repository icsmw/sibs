use std::path::PathBuf;

use crate::{
    elements::FuncArg,
    functions::{ExecutorFnDescription, ExecutorPinnedResult, TryAnyTo, E},
    inf::{Context, Scope, Store, Value, ValueRef},
};
use importer::import;

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    #[import(env)]
    fn var(key: String) -> Result<String, E> {
        Ok(std::env::var(key).unwrap_or_default())
    }
    #[import(env)]
    fn set_var(key: String, value: String) -> Result<(), E> {
        std::env::set_var(key, value);
        Ok(())
    }
    #[import(env)]
    fn remove_var(key: String) -> Result<(), E> {
        std::env::remove_var(key);
        Ok(())
    }
    #[import(env)]
    fn temp_dir() -> Result<PathBuf, E> {
        Ok(std::env::temp_dir())
    }
    #[import(env)]
    fn arch() -> Result<String, E> {
        Ok(std::env::consts::ARCH.to_string())
    }
    #[import(env)]
    fn os() -> Result<String, E> {
        Ok(std::env::consts::OS.to_string())
    }
    #[import(env)]
    fn is_os(os: String) -> Result<bool, E> {
        Ok(std::env::consts::OS.to_lowercase() == os.to_lowercase())
    }
    #[import(env)]
    fn family() -> Result<String, E> {
        Ok(std::env::consts::FAMILY.to_string())
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{
        elements::{ElementRef, Element},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    const TESTS: &[&str] = &[
        r#"if env::var(TEST_VAR) == "__test_var__" {"true";} else {"false";};"#,
        r#"if env::family() == "__family__" {"true";} else {"false";};"#,
        r#"if env::os() == "__os__" {"true";} else {"false";};"#,
        r#"if env::arch() == "__arch__" {"true";} else {"false";};"#,
        r#"if env::temp_dir() == "__temp_dir__" {"true";} else {"false";};"#,
        r#"env::remove_var(TEST_VAR); if env::var(TEST_VAR) == "" {"true";} else {"false";};"#,
        r#"env::set_var(TEST_VAR, "VALUE"); if env::var(TEST_VAR) == "VALUE" {"true";} else {"false";};"#,
    ];

    #[tokio::test]
    async fn reading() {
        std::env::set_var("TEST_VAR", "TEST");
        let temp_dir = std::env::temp_dir().to_string_lossy().to_string();
        let hooks: &[(&str, &str)] = &[
            ("__test_var__", "TEST"),
            ("__family__", std::env::consts::FAMILY),
            ("__os__", std::env::consts::OS),
            ("__arch__", std::env::consts::ARCH),
            ("__temp_dir__", temp_dir.as_ref()),
        ];
        fn apply_hooks(mut src: String, hooks: &[(&str, &str)]) -> String {
            hooks.iter().for_each(|(hook, value)| {
                src = src.replace(hook, value);
            });
            src
        }
        for test in TESTS.iter() {
            process_string!(
                &Configuration::logs(false),
                &apply_hooks(format!("@test{{{test}}}"), hooks),
                |reader: &mut Reader, src: &mut Sources| {
                    let mut tasks: Vec<Element> = Vec::new();
                    while let Some(task) =
                        src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                    {
                        let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                        tasks.push(task);
                    }
                    Ok::<Vec<Element>, LinkedErr<E>>(tasks)
                },
                |tasks: Vec<Element>, cx: Context, sc: Scope, journal: Journal| async move {
                    assert!(!tasks.is_empty());
                    for task in tasks.iter() {
                        let result = task
                            .execute(ExecuteContext::unbound(cx.clone(), sc.clone()))
                            .await;
                        if let Err(err) = result.as_ref() {
                            journal.report(err.into());
                        }
                        assert_eq!(
                            result
                                .expect("run of task is success")
                                .as_string()
                                .expect("test returns string value"),
                            "true".to_owned()
                        );
                    }
                    Ok::<(), LinkedErr<E>>(())
                }
            );
        }
    }
}
