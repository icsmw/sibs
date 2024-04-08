use crate::inf::{
    operator::OperatorResult,
    tracker::{logger::Logs, Tracker},
};

#[derive(Clone, Debug)]
pub struct Task {
    tracker: Tracker,
    id: usize,
    alias: String,
}

impl Task {
    pub fn new(tracker: &Tracker, id: usize, alias: &str) -> Self {
        Self {
            tracker: tracker.clone(),
            id,
            alias: alias.to_owned(),
        }
    }

    pub fn progress(&self, pos: Option<u64>) {
        self.tracker.progress(self.id, pos);
    }

    pub fn msg(&self, log: &str) {
        self.tracker.msg(self.id, log);
    }

    pub fn success(&self) {
        self.tracker.success(self.id);
    }

    pub fn fail(&self) {
        self.tracker.fail(self.id);
    }

    pub fn result(&self, result: OperatorResult) -> OperatorResult {
        match result.as_ref() {
            Ok(_) => self.success(),
            Err(err) => {
                self.err(err.to_string());
                self.fail();
            }
        };
        result
    }
}

impl Logs for Task {
    fn get_alias(&self) -> &str {
        &self.alias
    }
    fn get_tracker(&self) -> &Tracker {
        &self.tracker
    }
}
