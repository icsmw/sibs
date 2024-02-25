mod error;
use crate::{
    error::LinkedErr,
    executors::{self, ExecutorFn},
    inf::{
        any::AnyValue,
        scenario::Scenario,
        term::Term,
        tracker::{self, Logger, Logs, Tracker},
    },
    reader::map::Map,
};
pub use error::E;
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    fmt,
    path::{Path, PathBuf},
    rc::Rc,
};

#[derive(Debug)]
pub struct Context {
    pub cwd: Option<PathBuf>,
    pub term: Term,
    pub tracker: Tracker,
    pub scenario: Scenario,
    pub map: Rc<RefCell<Map>>,
    vars: HashMap<String, AnyValue>,
    executors: HashMap<String, ExecutorFn>,
    logger: Logger,
}

impl Context {
    fn register_functions(mut cx: Self) -> Result<Self, E> {
        executors::register(&mut cx)?;
        Ok(cx)
    }

    pub fn with_tracker(tracker: Tracker) -> Result<Self, E> {
        let logger = tracker.create_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: None,
            scenario: Scenario::dummy(),
            tracker,
            term: Term::new(),
            map: Map::new_wrapped(""),
            vars: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }
    pub fn from_filename(filename: &Path) -> Result<Self, E> {
        let tracker = Tracker::new(tracker::Configuration::default());
        let logger = tracker.create_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: Some(
                filename
                    .parent()
                    .ok_or(E::NoParentFolderFor(filename.to_string_lossy().to_string()))?
                    .to_path_buf(),
            ),
            scenario: Scenario::from(filename)?,
            tracker,
            map: Map::new_wrapped(""),
            term: Term::new(),
            vars: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }

    pub fn unbound() -> Result<Self, E> {
        let tracker = Tracker::new(tracker::Configuration::default());
        let logger = tracker.create_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: Some(PathBuf::new()),
            scenario: Scenario::dummy(),
            tracker,
            map: Map::new_wrapped(""),
            term: Term::new(),
            vars: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }
    pub fn get_map_ref(&self) -> Rc<RefCell<Map>> {
        self.map.clone()
    }
    pub fn gen_report<'a, T>(&self, token: &usize, msg: T) -> Result<(), E>
    where
        T: 'a + ToOwned + ToString,
    {
        self.map.borrow_mut().gen_report(token, msg)?;
        Ok(())
    }
    pub fn gen_report_from_err<T>(&self, err: &LinkedErr<T>) -> Result<(), E>
    where
        T: fmt::Display + ToString,
    {
        if let Some(token) = err.token.as_ref() {
            self.map.borrow_mut().gen_report(token, err.e.to_string())?;
        }
        Ok(())
    }
    pub fn set_map_cursor(&self, token: usize) {
        self.map.borrow_mut().set_cursor(token);
    }
    pub fn set_scenario(&mut self, scenario: Scenario) {
        self.scenario = scenario;
    }
    pub async fn set_cwd(&mut self, cwd: Option<PathBuf>) -> Result<(), E> {
        if let Some(cwd) = cwd.as_ref() {
            let cwd = self.scenario.to_abs_path(cwd)?;
            self.logger
                .log(format!("cwd: {}", cwd.to_string_lossy()))
                .await;
            self.cwd = Some(cwd);
        } else {
            self.cwd = None;
        }
        Ok(())
    }

    pub fn add_fn(&mut self, name: String, func: ExecutorFn) -> Result<(), E> {
        if let Entry::Vacant(e) = self.executors.entry(name.clone()) {
            e.insert(func);

            Ok(())
        } else {
            Err(E::FunctionAlreadyExists(name))
        }
    }

    pub fn get_fn(&self, name: &str) -> Option<&ExecutorFn> {
        self.executors.get(name)
    }

    pub async fn get_var(&self, name: &str) -> Option<&AnyValue> {
        self.logger
            .log(format!("Reading variable: ${name};",))
            .await;
        if !self.vars.contains_key(name) {
            self.logger
                .err(format!("Variable: ${name} doesn't exist;"))
                .await;
        }
        self.vars.get(name)
    }

    pub async fn set_var(&mut self, name: String, value: AnyValue) {
        self.logger
            .log(format!("Assignation: ${name} = {value}"))
            .await;
        self.vars.insert(name, value);
    }
}
