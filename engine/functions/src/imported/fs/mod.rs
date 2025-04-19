use crate::*;
use std::path::PathBuf;

pub fn register(efns: &mut EFns) -> Result<(), E> {
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
