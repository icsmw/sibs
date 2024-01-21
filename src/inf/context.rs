use crate::{
    inf::{
        any::{AnyValue, DebugAny},
        scenario::Scenario,
        term::Term,
        tracker::Tracker,
    },
    reader,
};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Context {
    pub cwd: Option<PathBuf>,
    pub term: Term,
    pub tracker: Tracker,
    pub scenario: Scenario,
    pub vars: HashMap<String, AnyValue>,
}

impl Context {
    pub fn new(term: Term, tracker: Tracker, scenario: Scenario) -> Self {
        Context {
            cwd: None,
            scenario,
            tracker,
            term,
            vars: HashMap::new(),
        }
    }
    pub fn from_filename(filename: &PathBuf) -> Result<Self, reader::error::E> {
        Ok(Context {
            cwd: Some(
                filename
                    .parent()
                    .ok_or(reader::error::E::NoFileParent)?
                    .to_path_buf(),
            ),
            scenario: Scenario::from(filename)?,
            tracker: Tracker::new(),
            term: Term::new(),
            vars: HashMap::new(),
        })
    }

    pub fn unbound() -> Self {
        Context {
            cwd: Some(PathBuf::new()),
            scenario: Scenario::dummy(),
            tracker: Tracker::new(),
            term: Term::new(),
            vars: HashMap::new(),
        }
    }

    pub fn set_cwd(&mut self, cwd: Option<PathBuf>) -> Result<(), reader::error::E> {
        if let Some(cwd) = cwd.as_ref() {
            self.cwd = Some(self.scenario.to_abs_path(cwd)?);
        } else {
            self.cwd = None;
        }
        Ok(())
    }
}
