use std::hash::Hash;

use tokio_util::sync::CancellationToken;

use crate::*;

#[derive(Debug, Clone)]
pub struct JobEntry {
    pub(crate) owner: Uuid,
    pub(crate) parent: Option<Uuid>,
    pub(crate) alias: String,
    pub(crate) childs: HashMap<Uuid, JobEntry>,
    pub cancel: CancellationToken,
}

impl JobEntry {
    pub fn new<S: ToString>(
        alias: S,
        owner: Uuid,
        parent: Option<Uuid>,
        cancel: CancellationToken,
    ) -> Self {
        Self {
            owner,
            parent,
            alias: alias.to_string(),
            childs: HashMap::new(),
            cancel,
        }
    }
    pub(crate) fn find(&mut self, uuid: &Uuid) -> Option<&mut JobEntry> {
        if &self.owner == uuid {
            return Some(self);
        }
        for (job_uuid, job) in self.childs.iter_mut() {
            if job_uuid == uuid {
                return Some(job);
            }
            if let Some(job) = job.find(uuid) {
                return Some(job);
            }
        }
        None
    }
    pub(crate) fn add_child(&mut self, job: &JobEntry) -> Result<(), E> {
        if self.childs.contains_key(&job.owner) {
            return Err(E::JobAlreadyExists(job.owner, job.alias.to_owned()));
        }
        self.childs.insert(job.owner, job.clone());
        Ok(())
    }
    pub(crate) fn as_job(&self, journal: Journal, progress: Progress, rt: RtJobs) -> Job {
        Job::new(
            self.alias.clone(),
            self.owner,
            self.parent,
            journal,
            progress,
            rt,
        )
    }
}
