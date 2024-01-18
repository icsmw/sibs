use crate::{
    inf::{reporter::Reporter, scenario::Scenario, tracker::Tracker},
    reader,
};
use std::path::PathBuf;

pub struct Context {
    pub cwd: PathBuf,
    pub reporter: Reporter,
    pub tracker: Tracker,
    pub scenario: Scenario,
}

impl Context {
    pub fn from_filename(filename: &PathBuf) -> Result<Self, reader::error::E> {
        Ok(Context {
            cwd: filename
                .parent()
                .ok_or(reader::error::E::NoFileParent)?
                .to_path_buf(),
            scenario: Scenario::from(filename)?,
            tracker: Tracker::new(),
            reporter: Reporter::new(),
        })
    }

    pub fn unbound() -> Self {
        Context {
            cwd: PathBuf::new(),
            scenario: Scenario::dummy(),
            tracker: Tracker::new(),
            reporter: Reporter::new(),
        }
    }
}
