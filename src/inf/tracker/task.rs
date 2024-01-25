use crate::inf::tracker::Tracker;

#[derive(Clone, Debug)]
pub struct Task {
    tracker: Tracker,
    id: usize,
}

impl Task {
    pub fn new(tracker: &Tracker, id: usize) -> Self {
        Self {
            tracker: tracker.clone(),
            id,
        }
    }

    pub async fn progress(&self, pos: Option<u64>) {
        self.tracker.progress(self.id, pos).await;
    }

    pub async fn msg(&self, log: &str) {
        self.tracker.msg(self.id, log).await;
    }

    pub async fn success(&self, msg: &str) {
        self.tracker.success(self.id, msg).await;
    }

    pub async fn fail(&self, msg: &str) {
        self.tracker.fail(self.id, msg).await;
    }
}
