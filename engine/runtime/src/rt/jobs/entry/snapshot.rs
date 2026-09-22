use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct JobFinishToken {
    inner: Arc<AtomicBool>,
}

impl JobFinishToken {
    pub fn is_done(&self) -> bool {
        self.inner.load(Ordering::Relaxed)
    }
    pub fn done(&self) {
        self.inner.store(true, Ordering::Relaxed);
    }
}

pub struct JobStateSnapshot {
    token: JobFinishToken,
    uuid: Uuid,
    pub(super) childs: HashMap<Uuid, JobStateSnapshot>,
}

impl JobStateSnapshot {
    pub fn new(token: JobFinishToken, uuid: Uuid) -> Self {
        Self {
            token,
            uuid,
            childs: HashMap::new(),
        }
    }
    pub fn unfinished(&self) -> Vec<Uuid> {
        fn collect(entry: &JobStateSnapshot, out: &mut Vec<Uuid>) {
            for child in entry.childs.values() {
                if !child.token.is_done() {
                    out.push(child.uuid);
                }
                collect(child, out);
            }
        }
        let mut out = Vec::new();
        collect(self, &mut out);
        out
    }
}
