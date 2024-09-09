use crate::inf::{Journal, OwnedJournal, Value};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Sessions {
    sessions: HashMap<Uuid, Session>,
}

impl Sessions {
    pub fn add<S: AsRef<str>>(&mut self, alias: S, cwd: PathBuf, journal: &Journal) -> Uuid {
        let uuid = Uuid::new_v4();
        self.sessions
            .insert(uuid, Session::new(alias, cwd, journal));
        uuid
    }
    pub fn remove(&mut self, uuid: Uuid) {
        self.sessions.remove(&uuid);
    }
    pub fn get(&mut self, uuid: &Uuid) -> Option<&mut Session> {
        self.sessions.get_mut(uuid)
    }
}

#[derive(Debug)]
pub struct Session {
    vars: HashMap<String, Arc<Value>>,
    cwd: PathBuf,
    loops: Vec<(Uuid, CancellationToken)>,
    journal: OwnedJournal,
}

impl Session {
    pub fn new<S: AsRef<str>>(alias: S, cwd: PathBuf, journal: &Journal) -> Self {
        Self {
            vars: HashMap::new(),
            cwd,
            loops: Vec::new(),
            journal: journal.owned(alias.as_ref(), None),
        }
    }

    pub fn set_var(&mut self, key: &str, value: Value) -> bool {
        self.vars.insert(key.to_string(), Arc::new(value)).is_some()
    }

    pub fn get_var(&self, key: &str) -> Option<Arc<Value>> {
        self.vars.get(key).cloned()
    }

    pub fn set_cwd(&mut self, cwd: PathBuf) {
        self.journal.info(format!("set CWD to: {}", cwd.display()));
        self.cwd = cwd;
    }

    pub fn get_cwd(&self) -> PathBuf {
        self.cwd.clone()
    }

    pub fn open_loop(&mut self) -> (Uuid, CancellationToken) {
        let token = CancellationToken::new();
        let uuid = Uuid::new_v4();
        self.loops.push((uuid, token.clone()));
        (uuid, token)
    }

    pub fn close_loop(&mut self, uuid: Uuid) {
        self.loops.iter().for_each(|(id, token)| {
            if id == &uuid {
                token.cancel();
            }
        });
        self.loops.retain(|(id, _)| id != &uuid);
    }

    pub fn break_loop(&mut self) -> bool {
        self.loops
            .pop()
            .map(|(_, token)| {
                token.cancel();
                true
            })
            .unwrap_or(false)
    }
}
