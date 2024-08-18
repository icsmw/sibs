use crate::{
    elements::FuncArg,
    functions::{ExecutorFn, ExecutorPinnedResult, TryAnyTo, E},
    inf::{AnyValue, Context, Scope, Store},
};
use importer::import;

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    #[import(fs)]
    fn create_dir(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::Empty(std::fs::create_dir(path)?))
    }
    #[import(fs)]
    fn create_dir_all(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::Empty(std::fs::create_dir_all(path)?))
    }
    #[import(fs)]
    fn remove_dir(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::Empty(std::fs::remove_dir(path)?))
    }
    #[import(fs)]
    fn remove_dir_all(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::Empty(std::fs::remove_dir_all(path)?))
    }
    #[import(fs)]
    fn remove_file(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::Empty(std::fs::remove_file(path)?))
    }
    #[import(fs)]
    fn rename(a: std::path::PathBuf, b: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::Empty(std::fs::rename(a, b)?))
    }
    #[import(fs)]
    fn copy(a: std::path::PathBuf, b: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::u64(std::fs::copy(a, b)?))
    }
    #[import(fs)]
    fn hard_link(a: std::path::PathBuf, b: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::Empty(std::fs::hard_link(a, b)?))
    }
    #[import(fs)]
    fn canonicalize(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::PathBuf(std::fs::canonicalize(path)?))
    }
    #[import(fs)]
    fn read_to_string(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::String(std::fs::read_to_string(path)?))
    }
    #[import(fs)]
    fn write(path: std::path::PathBuf, data: String) -> Result<AnyValue, E> {
        Ok(AnyValue::Empty(std::fs::write(path, data)?))
    }
    #[import(fs)]
    fn append(path: std::path::PathBuf, data: String) -> Result<AnyValue, E> {
        use std::fs::OpenOptions;
        use std::io::Write;
        let mut file = OpenOptions::new().append(true).open(path)?;
        Ok(AnyValue::Empty(writeln!(file, "{data}")?))
    }
    #[import(fs)]
    fn is_file(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::bool(std::fs::metadata(path)?.is_file()))
    }
    #[import(fs)]
    fn is_dir(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::bool(std::fs::metadata(path)?.is_dir()))
    }
    #[import(fs)]
    fn exists(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::bool(path.exists()))
    }
    #[import(fs)]
    fn is_absolute(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::bool(path.is_absolute()))
    }
    #[import(fs)]
    fn is_relative(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::bool(path.is_relative()))
    }
    #[import(fs)]
    fn is_symlink(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::bool(std::fs::metadata(path)?.is_symlink()))
    }
    #[import(fs)]
    fn file_size(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::u64(std::fs::metadata(path)?.len()))
    }
    #[import(fs)]
    fn file_created_timestamp(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::u128(
            std::fs::metadata(path)?
                .created()?
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis(),
        ))
    }
    #[import(fs)]
    fn file_modified_timestamp(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::u128(
            std::fs::metadata(path)?
                .modified()?
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis(),
        ))
    }
    #[import(fs)]
    fn file_accessed_timestamp(path: std::path::PathBuf) -> Result<AnyValue, E> {
        Ok(AnyValue::u128(
            std::fs::metadata(path)?
                .accessed()?
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis(),
        ))
    }
    #[import(fs)]
    fn path_join(paths: Vec<std::path::PathBuf>) -> Result<AnyValue, E> {
        let mut path = std::path::PathBuf::new();
        paths.iter().for_each(|part| {
            path.push(part);
        });
        Ok(AnyValue::PathBuf(path))
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use journal::Journal;
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::Task,
        error::LinkedErr,
        inf::{
            journal,
            operator::{Execute, E},
            Configuration, Context, Scope,
        },
        process_string,
        reader::{chars, Dissect, Reader, Sources},
    };

    const TESTS: &[&str] = &[
        r#"$tmp_path = env::temp_dir(); $file_name = "test.txt"; $file = fs::path_join(($tmp_path; $file_name)); if $file == "__temp_file__" ["true";] else ["false";];"#,
        // r#"$file = fs::path_join(env::temp_dir(); "test.txt"); if $file == "__temp_file__" ["true";] else ["false";];"#,
    ];

    #[tokio::test]
    async fn reading() {
        std::env::set_var("TEST_VAR", "TEST");
        let temp_file = std::env::temp_dir()
            .join("test.txt")
            .to_string_lossy()
            .to_string();
        let hooks: &[(&str, &str)] = &[("__temp_file__", &temp_file)];
        fn apply_hooks(mut src: String, hooks: &[(&str, &str)]) -> String {
            hooks.iter().for_each(|(hook, value)| {
                src = src.replace(hook, value);
            });
            src
        }
        for test in TESTS.iter() {
            process_string!(
                &Configuration::logs(false),
                &apply_hooks(format!("test[{test}]"), hooks),
                |reader: &mut Reader, src: &mut Sources| {
                    let mut tasks: Vec<Task> = Vec::new();
                    while let Some(task) = src.report_err_if(Task::dissect(reader))? {
                        let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                        tasks.push(task);
                    }
                    Ok::<Vec<Task>, LinkedErr<E>>(tasks)
                },
                |tasks: Vec<Task>, cx: Context, sc: Scope, journal: Journal| async move {
                    for task in tasks.iter() {
                        let result = task
                            .execute(
                                None,
                                &[],
                                &[],
                                cx.clone(),
                                sc.clone(),
                                CancellationToken::new(),
                            )
                            .await;
                        if let Err(err) = result.as_ref() {
                            journal.report(err.into());
                        }
                        assert_eq!(
                            result
                                .expect("run of task is success")
                                .expect("test returns some value")
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
