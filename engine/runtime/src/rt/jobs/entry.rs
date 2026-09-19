use super::*;
use crate::{
    rt::jobs::state::{JobState, JobStateError},
    *,
};

#[derive(Debug)]
pub struct JobEntry {
    identity: JobIdentity,
    childs: HashMap<Uuid, JobEntry>,
    sensors: JobSensonrs,
    state: JobState,
}

impl JobEntry {
    pub fn new(identity: JobIdentity) -> Self {
        Self {
            identity,
            childs: HashMap::new(),
            sensors: JobSensonrs::default(),
            state: JobState::default(),
        }
    }
    pub fn identity(&self) -> &JobIdentity {
        &self.identity
    }
    pub fn find(&mut self, uuid: &Uuid) -> Option<&mut JobEntry> {
        if &self.identity.uuid() == uuid {
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
    pub fn child<S: ToString>(&mut self, alias: S) -> Result<&JobEntry, E> {
        if !match self.state() {
            JobState::Created | JobState::Started(_) => true,
            JobState::Cancelling
            | JobState::Cancelled(_)
            | JobState::Failed(_)
            | JobState::Success(_) => false,
        } {
            return Err(
                JobStateError::InvalidState(self.identity.uuid(), self.state().clone()).into(),
            );
        }
        let parent = self.identity.uuid();
        let job = Self {
            identity: JobIdentity::new(alias, Some(parent.clone())),
            childs: HashMap::new(),
            sensors: self.sensors.child(),
            state: JobState::default(),
        };
        if self.childs.contains_key(&job.identity.uuid()) {
            return Err(E::JobAlreadyExists(
                job.identity.uuid(),
                job.identity.alias().to_string(),
            ));
        }
        self.childs.insert(job.identity.uuid(), job);
        self.childs.get(&parent).ok_or(E::JobDoesNotExist(parent))
    }
    pub fn job(&self, jobs: RtJobs, journal: RtJournal, progress: RtProgress) -> Job {
        Job::new(
            self.identity.clone(),
            self.sensors.clone(),
            journal,
            progress,
            jobs,
        )
    }
    pub fn state(&self) -> &JobState {
        &self.state
    }
    pub fn update(&mut self, state: JobState) -> Result<(), E> {
        self.state.update(self.identity.uuid(), state.clone())?;
        match state {
            JobState::Cancelling => {
                self.sensors.cancel();
            }
            JobState::Success(_)
            | JobState::Failed(_)
            | JobState::Cancelled(_)
            | JobState::Created
            | JobState::Started(_) => {}
        }
        Ok(())
    }
}
