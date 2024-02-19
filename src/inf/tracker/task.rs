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

    pub async fn progress(&self, pos: Option<u64>) {
        self.tracker.progress(self.id, pos).await;
    }

    pub async fn msg(&self, log: &str) {
        self.tracker.msg(self.id, log).await;
    }

    pub async fn success(&self) {
        self.tracker.success(self.id).await;
    }

    pub async fn fail(&self) {
        self.tracker.fail(self.id).await;
    }

    pub async fn result(&self, result: OperatorResult) -> OperatorResult {
        match result.as_ref() {
            Ok(_) => self.success().await,
            Err(err) => {
                self.err(err.to_string()).await;
                self.fail().await;
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
