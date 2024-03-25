use crate::{
    executors::{ExecutorPinnedResult, TryAnyTo, E},
    inf::{any::AnyValue, context::Context},
};
use importer::import;

pub fn register(cx: &mut Context) -> Result<(), E> {
    #[import(fs)]
    fn create_dir(path: std::path::PathBuf) -> Result<(), E> {
        Ok(std::fs::create_dir(path)?)
    }
    #[import(fs)]
    fn create_dir_all(path: std::path::PathBuf) -> Result<(), E> {
        Ok(std::fs::create_dir_all(path)?)
    }
    #[import(fs)]
    fn remove_dir(path: std::path::PathBuf) -> Result<(), E> {
        Ok(std::fs::remove_dir(path)?)
    }
    #[import(fs)]
    fn remove_dir_all(path: std::path::PathBuf) -> Result<(), E> {
        Ok(std::fs::remove_dir_all(path)?)
    }
    #[import(fs)]
    fn remove_file(path: std::path::PathBuf) -> Result<(), E> {
        Ok(std::fs::remove_file(path)?)
    }
    #[import(fs)]
    fn rename(a: std::path::PathBuf, b: std::path::PathBuf) -> Result<(), E> {
        Ok(std::fs::rename(a, b)?)
    }
    #[import(fs)]
    fn copy(a: std::path::PathBuf, b: std::path::PathBuf) -> Result<u64, E> {
        Ok(std::fs::copy(a, b)?)
    }
    #[import(fs)]
    fn hard_link(a: std::path::PathBuf, b: std::path::PathBuf) -> Result<(), E> {
        Ok(std::fs::hard_link(a, b)?)
    }
    #[import(fs)]
    fn canonicalize(path: std::path::PathBuf) -> Result<std::path::PathBuf, E> {
        Ok(std::fs::canonicalize(path)?)
    }
    #[import(fs)]
    fn read_to_string(path: std::path::PathBuf) -> Result<String, E> {
        Ok(std::fs::read_to_string(path)?)
    }
    #[import(fs)]
    fn write(path: std::path::PathBuf, data: String) -> Result<(), E> {
        Ok(std::fs::write(path, data)?)
    }
    #[import(fs)]
    fn append(path: std::path::PathBuf, data: String) -> Result<(), E> {
        use std::fs::OpenOptions;
        use std::io::Write;
        let mut file = OpenOptions::new().append(true).open(path)?;
        Ok(writeln!(file, "{data}")?)
    }
    #[import(fs)]
    fn is_file(path: std::path::PathBuf) -> Result<bool, E> {
        Ok(std::fs::metadata(path)?.is_file())
    }
    #[import(fs)]
    fn is_dir(path: std::path::PathBuf) -> Result<bool, E> {
        Ok(std::fs::metadata(path)?.is_dir())
    }
    #[import(fs)]
    fn exists(path: std::path::PathBuf) -> Result<bool, E> {
        Ok(path.exists())
    }
    #[import(fs)]
    fn is_absolute(path: std::path::PathBuf) -> Result<bool, E> {
        Ok(path.is_absolute())
    }
    #[import(fs)]
    fn is_relative(path: std::path::PathBuf) -> Result<bool, E> {
        Ok(path.is_relative())
    }
    #[import(fs)]
    fn is_symlink(path: std::path::PathBuf) -> Result<bool, E> {
        Ok(std::fs::metadata(path)?.is_symlink())
    }
    #[import(fs)]
    fn file_size(path: std::path::PathBuf) -> Result<u64, E> {
        Ok(std::fs::metadata(path)?.len())
    }
    #[import(fs)]
    fn file_created_timestamp(path: std::path::PathBuf) -> Result<u128, E> {
        Ok(std::fs::metadata(path)?
            .created()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis())
    }
    #[import(fs)]
    fn file_modified_timestamp(path: std::path::PathBuf) -> Result<u128, E> {
        Ok(std::fs::metadata(path)?
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis())
    }
    #[import(fs)]
    fn file_accessed_timestamp(path: std::path::PathBuf) -> Result<u128, E> {
        Ok(std::fs::metadata(path)?
            .accessed()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis())
    }
    #[import(fs)]
    fn path_join(paths: Vec<std::path::PathBuf>) -> Result<std::path::PathBuf, E> {
        let mut path = std::path::PathBuf::new();
        paths.iter().for_each(|part| {
            path.push(part);
        });
        Ok(path)
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{
        elements::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
            tests::*,
        },
        reader::{chars, Reading},
    };

    const TESTS: &[&str] = &[
        r#"$tmp_path = @env::temp_dir; $file_name = "test.txt"; $file = @fs::path_join(($tmp_path; $file_name)); IF $file == "__temp_file__" ["true";] ELSE ["false";];"#,
        // r#"$file = @fs::path_join(@env::temp_dir; "test.txt"); IF $file == "__temp_file__" ["true";] ELSE ["false";];"#,
    ];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
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
            let mut cx = Context::create().unbound()?;
            let mut reader = cx
                .reader()
                .from_str(&apply_hooks(format!("test[{test}]"), hooks));
            while let Some(task) = Task::read(&mut reader)? {
                let result = task.execute(None, &[], &[], &mut cx).await;
                let result = post_if_err(&cx, result)?.expect("test returns some value");
                let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                assert_eq!(
                    result.get_as_string().expect("test returns string value"),
                    "true".to_owned()
                );
            }
        }

        Ok(())
    }
}
