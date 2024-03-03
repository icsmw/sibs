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
    Ok(())
}
