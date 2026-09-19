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
    pub fn path(
        &self,
        uuid: Uuid,
        filter: &dyn Fn(&JobIdentity) -> bool,
    ) -> Result<Vec<JobElement>, E> {
        fn locate<'a>(entry: &'a JobEntry, uuid: Uuid, path: &mut Vec<&'a JobIdentity>) -> bool {
            if entry.identity.uuid() == uuid
                || entry.childs.values().any(|child| locate(child, uuid, path))
            {
                path.push(&entry.identity);
                true
            } else {
                false
            }
        }
        let mut path = Vec::new();
        if !locate(self, uuid, &mut path) {
            return Err(E::JobDoesNotExist(uuid));
        }
        // Filter only the actual path, in root-to-target order. No identities
        // or labels from unrelated branches are cloned.
        Ok(path
            .into_iter()
            .rev()
            .filter(|identity| filter(identity))
            .map(JobElement::from)
            .collect())
    }

    pub fn child<S: ToString>(
        &mut self,
        alias: S,
        visibility: JobVisibility,
    ) -> Result<&JobEntry, E> {
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
            identity: JobIdentity::new(alias, Some(parent.clone()), visibility),
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
        let uuid = job.identity.uuid();
        self.childs.insert(uuid, job);
        self.childs.get(&uuid).ok_or(E::JobDoesNotExist(uuid))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_filters_only_the_ancestor_chain_in_order() {
        let mut root = JobEntry::new(JobIdentity::new("root", None, JobVisibility::Visible));
        let hidden = root
            .child("hidden", JobVisibility::Hidden)
            .unwrap()
            .identity()
            .uuid();
        let target = root
            .find(&hidden)
            .unwrap()
            .child("target", JobVisibility::Visible)
            .unwrap()
            .identity()
            .uuid();
        root.child("unrelated", JobVisibility::Visible).unwrap();
        let visited = std::cell::RefCell::new(Vec::new());
        let path = root
            .path(target, &|identity| {
                visited.borrow_mut().push(identity.alias().to_owned());
                matches!(identity.visibility(), JobVisibility::Visible)
            })
            .unwrap();
        assert_eq!(*visited.borrow(), ["root", "hidden", "target"]);
        assert_eq!(
            path,
            vec![
                JobElement::from(root.identity()),
                JobElement::from(root.find(&target).unwrap().identity())
            ]
        );
        assert!(root.path(target, &|_| false).unwrap().is_empty());
        assert_eq!(
            root.path(hidden, &|identity| matches!(
                identity.visibility(),
                JobVisibility::Visible
            ))
            .unwrap(),
            vec![JobElement::from(root.identity())]
        );
        assert_eq!(
            root.path(root.identity().uuid(), &|_| true).unwrap(),
            vec![JobElement::from(root.identity())]
        );
        let missing = Uuid::new_v4();
        assert!(
            matches!(root.path(missing, &|_| panic!("filter must not run for missing target")), Err(E::JobDoesNotExist(id)) if id == missing)
        );
    }

    #[test]
    fn child_returns_the_registered_entry() {
        let mut parent = JobEntry::new(JobIdentity::new("parent", None, JobVisibility::default()));
        let parent_uuid = parent.identity().uuid();
        let first_uuid = {
            let child = parent
                .child("child", JobVisibility::default())
                .expect("child created");
            assert_eq!(child.identity().parent(), Some(parent_uuid));
            assert_eq!(child.identity().alias(), "child");
            assert_eq!(child.state(), &JobState::Created);
            child.identity().uuid()
        };
        let second_uuid = parent
            .child("child", JobVisibility::default())
            .expect("second child created")
            .identity()
            .uuid();
        assert_ne!(first_uuid, parent_uuid);
        assert_ne!(first_uuid, second_uuid);
        assert_eq!(parent.childs.len(), 2);
        assert_eq!(
            parent.find(&first_uuid).unwrap().identity().uuid(),
            first_uuid
        );
        assert_eq!(
            parent.find(&second_uuid).unwrap().identity().uuid(),
            second_uuid
        );
    }
}
