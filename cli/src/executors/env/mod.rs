use crate::{
    executors::Store,
    executors::{ExecutorPinnedResult, TryAnyTo, E},
    inf::{any::AnyValue, context::Context},
};
use importer::import;

pub fn register(store: &mut Store) -> Result<(), E> {
    #[import(env)]
    fn var(key: String) -> Result<String, E> {
        Ok(match std::env::var(key) {
            Ok(v) => v,
            Err(_) => String::from(""),
        })
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
    fn temp_dir() -> Result<std::path::PathBuf, E> {
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
    fn family() -> Result<String, E> {
        Ok(std::env::consts::FAMILY.to_string())
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{
        elements::Task,
        error::LinkedErr,
        inf::{
            operator::{Operator, E},
            tests::*,
        },
        reader::{chars, Reading, Sources},
    };

    const TESTS: &[&str] = &[
        r#"IF @env::var(TEST_VAR) == "__test_var__" ["true";] ELSE ["false";];"#,
        r#"IF @env::family == "__family__" ["true";] ELSE ["false";];"#,
        r#"IF @env::os == "__os__" ["true";] ELSE ["false";];"#,
        r#"IF @env::arch == "__arch__" ["true";] ELSE ["false";];"#,
        r#"IF @env::temp_dir == "__temp_dir__" ["true";] ELSE ["false";];"#,
        r#"@env::remove_var(TEST_VAR); IF @env::var(TEST_VAR) == "" ["true";] ELSE ["false";];"#,
        r#"@env::set_var(TEST_VAR; "VALUE"); IF @env::var(TEST_VAR) == "VALUE" ["true";] ELSE ["false";];"#,
    ];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
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
            let (tasks, mut src): (Vec<Task>, Sources) = runner(
                &apply_hooks(format!("test[{test}]"), hooks),
                |src, mut reader| {
                    let mut tasks: Vec<Task> = Vec::new();
                    while let Some(task) = Task::read(&mut reader)? {
                        let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                        tasks.push(task);
                    }
                    Ok::<(Vec<Task>, Sources), LinkedErr<E>>((tasks, src))
                },
            )
            .await?;
            for task in tasks.iter() {
                let result = execution(&src, |cx, sc| {
                    Box::pin(async move { task.execute(None, &[], &[], cx, sc).await })
                })
                .await;
                let value = src.report_err_if(result)?.expect("test returns some value");
                assert_eq!(
                    value.get_as_string().expect("test returns string value"),
                    "true".to_owned()
                );
            }
        }

        Ok(())
    }
}
