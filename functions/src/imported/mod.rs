use std::path::PathBuf;

use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    #[import(fs)]
    fn create_dir(path: PathBuf) -> Result<(), E> {
        let _ = std::fs::create_dir(path);
        Ok(())
    }
    Ok(())
}
