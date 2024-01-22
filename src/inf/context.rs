use crate::{
    functions::{self, register, FunctionExecutor},
    inf::{
        any::{AnyValue, DebugAny},
        scenario::Scenario,
        term::Term,
        tracker::Tracker,
    },
    reader,
};
use std::{
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Context {
    pub cwd: Option<PathBuf>,
    pub term: Term,
    pub tracker: Tracker,
    pub scenario: Scenario,
    pub vars: HashMap<String, AnyValue>,
    pub processed: HashMap<Uuid, AnyValue>,
    pub functions: HashMap<String, FunctionExecutor>,
}

impl Context {
    fn register_functions(mut cx: Self) -> Result<Self, reader::error::E> {
        functions::register(&mut cx)?;
        Ok(cx)
    }

    pub fn new(term: Term, tracker: Tracker, scenario: Scenario) -> Result<Self, reader::error::E> {
        Self::register_functions(Context {
            cwd: None,
            scenario,
            tracker,
            term,
            vars: HashMap::new(),
            processed: HashMap::new(),
            functions: HashMap::new(),
        })
    }
    pub fn from_filename(filename: &PathBuf) -> Result<Self, reader::error::E> {
        Self::register_functions(Context {
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
            processed: HashMap::new(),
            functions: HashMap::new(),
        })
    }

    pub fn unbound() -> Result<Self, reader::error::E> {
        Self::register_functions(Context {
            cwd: Some(PathBuf::new()),
            scenario: Scenario::dummy(),
            tracker: Tracker::new(),
            term: Term::new(),
            vars: HashMap::new(),
            processed: HashMap::new(),
            functions: HashMap::new(),
        })
    }

    pub fn set_cwd(&mut self, cwd: Option<PathBuf>) -> Result<(), reader::error::E> {
        if let Some(cwd) = cwd.as_ref() {
            self.cwd = Some(self.scenario.to_abs_path(cwd)?);
        } else {
            self.cwd = None;
        }
        Ok(())
    }

    pub fn add_fn(&mut self, name: String, func: FunctionExecutor) -> Result<(), reader::error::E> {
        if let Entry::Vacant(e) = self.functions.entry(name.clone()) {
            e.insert(func);
            Ok(())
        } else {
            Err(reader::error::E::FunctionAlreadyExists(name))
        }
    }

    pub fn get_fn(&self, name: &str) -> Option<&FunctionExecutor> {
        self.functions.get(name)
    }
}
