use std::path::PathBuf;

use crate::*;

pub trait GetFilename {
    fn get_filename(&self) -> Result<PathBuf, E>;
}
