use crate::{
    inf::{location::Location, reporter::Reporter, tracker::Tracker},
    reader,
};
use std::path::PathBuf;

pub struct Context {
    pub cwd: PathBuf,
    pub reporter: Reporter,
    pub tracker: Tracker,
    pub location: Location,
}

impl Context {
    pub fn from_filename(filename: &PathBuf) -> Result<Self, reader::error::E> {
        Ok(Context {
            cwd: filename
                .parent()
                .ok_or(reader::error::E::NoFileParent)?
                .to_path_buf(),
            location: Location::from(filename)?,
            tracker: Tracker::new(),
            reporter: Reporter::new(),
        })
    }

    pub fn unbound() -> Self {
        Context {
            cwd: PathBuf::new(),
            location: Location::dummy(),
            tracker: Tracker::new(),
            reporter: Reporter::new(),
        }
    }
}
