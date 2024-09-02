use crate::{
    elements::FuncArg,
    functions::{ExecutorFnDescription, ExecutorPinnedResult, TryAnyTo, E},
    inf::{Context, Scope, Store, Value, ValueRef},
};
use importer::import;
use std::path::PathBuf;

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    #[import(fs)]
    fn create_dir(path: PathBuf) -> Result<(), E> {
        Ok(std::fs::create_dir(path)?)
    }
    #[import(fs)]
    fn create_dir_all(path: PathBuf) -> Result<(), E> {
        Ok(std::fs::create_dir_all(path)?)
    }
    #[import(fs)]
    fn remove_dir(path: PathBuf) -> Result<(), E> {
        Ok(std::fs::remove_dir(path)?)
    }
    #[import(fs)]
    fn remove_dir_all(path: PathBuf) -> Result<(), E> {
        Ok(std::fs::remove_dir_all(path)?)
    }
    #[import(fs)]
    fn remove_file(path: PathBuf) -> Result<(), E> {
        Ok(std::fs::remove_file(path)?)
    }
    #[import(fs)]
    fn rename(a: PathBuf, b: PathBuf) -> Result<(), E> {
        Ok(std::fs::rename(a, b)?)
    }
    #[import(fs)]
    fn copy(a: PathBuf, b: PathBuf) -> Result<u64, E> {
        Ok(std::fs::copy(a, b)?)
    }
    #[import(fs)]
    fn hard_link(a: PathBuf, b: PathBuf) -> Result<(), E> {
        Ok(std::fs::hard_link(a, b)?)
    }
    #[import(fs)]
    fn canonicalize(path: PathBuf) -> Result<PathBuf, E> {
        Ok(std::fs::canonicalize(path)?)
    }
    #[import(fs)]
    fn read_to_string(path: PathBuf) -> Result<String, E> {
        Ok(std::fs::read_to_string(path)?)
    }
    #[import(fs)]
    fn write(path: PathBuf, data: String) -> Result<(), E> {
        Ok(std::fs::write(path, data)?)
    }
    #[import(fs)]
    fn append(path: PathBuf, data: String) -> Result<(), E> {
        use std::fs::OpenOptions;
        use std::io::Write;
        let mut file = OpenOptions::new().append(true).open(path)?;
        Ok(writeln!(file, "{data}")?)
    }
    #[import(fs)]
    fn is_file(path: PathBuf) -> Result<bool, E> {
        Ok(std::fs::metadata(path)?.is_file())
    }
    #[import(fs)]
    fn is_dir(path: PathBuf) -> Result<bool, E> {
        Ok(std::fs::metadata(path)?.is_dir())
    }
    #[import(fs)]
    fn exists(path: PathBuf) -> Result<bool, E> {
        Ok(path.exists())
    }
    #[import(fs)]
    fn is_absolute(path: PathBuf) -> Result<bool, E> {
        Ok(path.is_absolute())
    }
    #[import(fs)]
    fn is_relative(path: PathBuf) -> Result<bool, E> {
        Ok(path.is_relative())
    }
    #[import(fs)]
    fn is_symlink(path: PathBuf) -> Result<bool, E> {
        Ok(std::fs::metadata(path)?.is_symlink())
    }
    #[import(fs)]
    fn file_size(path: PathBuf) -> Result<u64, E> {
        Ok(std::fs::metadata(path)?.len())
    }
    #[import(fs)]
    fn file_created_timestamp(path: PathBuf) -> Result<u128, E> {
        Ok(std::fs::metadata(path)?
            .created()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis())
    }
    #[import(fs)]
    fn file_modified_timestamp(path: PathBuf) -> Result<u128, E> {
        Ok(std::fs::metadata(path)?
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis())
    }
    #[import(fs)]
    fn file_accessed_timestamp(path: PathBuf) -> Result<u128, E> {
        Ok(std::fs::metadata(path)?
            .accessed()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis())
    }
    #[import(fs)]
    fn path_join(paths: Vec<PathBuf>) -> Result<PathBuf, E> {
        let mut path = PathBuf::new();
        paths.iter().for_each(|part| {
            path.push(part);
        });
        Ok(path)
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use journal::Journal;
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::{ElTarget, Element},
        error::LinkedErr,
        inf::{
            journal,
            operator::{Execute, E},
            Configuration, Context, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
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
                    let mut tasks: Vec<Element> = Vec::new();
                    while let Some(task) =
                        src.report_err_if(Element::include(reader, &[ElTarget::Task]))?
                    {
                        let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                        tasks.push(task);
                    }
                    Ok::<Vec<Element>, LinkedErr<E>>(tasks)
                },
                |tasks: Vec<Element>, cx: Context, sc: Scope, journal: Journal| async move {
                    for task in tasks.iter() {
                        let result = task
                            .execute(
                                None,
                                &[],
                                &[],
                                &None,
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
