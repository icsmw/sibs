mod error;
mod extentions;

use crate::{
    executors::ExecutorFn,
    inf::{AnyValue, Logger, Logs, Scenario, Term, Tracker},
    reader::sources::Sources,
};
pub use error::E;
use extentions::*;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Context {
    pub cwd: Option<PathBuf>,
    pub term: Term,
    pub tracker: Tracker,
    pub scenario: Scenario,
    pub sources: Sources,
    variables: HashMap<String, AnyValue>,
    executors: HashMap<String, ExecutorFn>,
    logger: Logger,
}

impl Context {
    pub fn create() -> Create {
        Create
    }
    pub fn reader(&mut self) -> ReaderGetter {
        ReaderGetter::new(self)
    }
    pub fn vars(&mut self) -> Vars {
        Vars::new(self)
    }
    pub fn functions(&mut self) -> Functions {
        Functions::new(self)
    }
    pub fn set_scenario(&mut self, scenario: Scenario) {
        self.scenario = scenario;
    }
    pub fn set_cwd(&mut self, cwd: Option<PathBuf>) -> Result<(), E> {
        if let Some(cwd) = cwd.as_ref() {
            let cwd = self.scenario.to_abs_path(cwd)?;
            self.logger.log(format!("cwd: {}", cwd.to_string_lossy()));
            self.cwd = Some(cwd);
        } else {
            self.cwd = None;
        }
        Ok(())
    }
}
