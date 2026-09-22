mod identity;
mod sensors;
mod snapshot;

use super::*;
use crate::{
    rt::jobs::state::{JobState, JobStateError},
    *,
};

pub use identity::*;
pub(crate) use sensors::*;
pub(crate) use snapshot::*;

pub(crate) type JobStateUpdate = Arc<Notify>;

#[derive(Debug)]
pub struct JobEntry {
    identity: JobIdentity,
    childs: HashMap<Uuid, JobEntry>,
    sensors: JobSensonrs,
    state: JobState,
    state_update: JobStateUpdate,
}

impl JobEntry {
    pub fn new(identity: JobIdentity, state_update: JobStateUpdate) -> Self {
        Self {
            identity,
            childs: HashMap::new(),
            sensors: JobSensonrs::default(),
            state: JobState::default(),
            state_update,
        }
    }
    pub(super) fn is_locked(&self) -> bool {
        self.state.is_locked()
    }
    pub(super) fn lock(&mut self) -> Result<(), E> {
        self.sensors.cancel();
        self.state
            .lock(self.identity.uuid())
            .map_err(|err| err.into())
    }

    pub(super) fn snapshot(&self) -> JobStateSnapshot {
        let mut snapshot = JobStateSnapshot::new(self.sensors.finished(), self.identity.uuid());
        for (uuid, entry) in self.childs.iter() {
            snapshot.childs.insert(*uuid, entry.snapshot());
        }
        snapshot
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
        if self.sensors.is_cancelled() {
            return Err(E::Cancelled);
        }
        if self.state.is_locked() {
            return Err(
                JobStateError::InvalidState(self.identity.uuid(), self.state.clone()).into(),
            );
        }
        let parent = self.identity.uuid();
        let job = Self {
            identity: JobIdentity::new(alias, Some(parent), visibility),
            childs: HashMap::new(),
            sensors: self.sensors.child(),
            state: JobState::default(),
            state_update: self.state_update.clone(),
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
        fn nested(state: &JobEntry) -> Option<&JobEntry> {
            state.childs.values().find_map(|child| {
                if !child.state.is_finished() {
                    Some(child)
                } else {
                    nested(child)
                }
            })
        }

        self.state.would_update(self.identity.uuid(), &state)?;

        // Validate the transition without committing it or triggering sensors.
        if state.is_finished() {
            if let Some(child) = nested(self) {
                return Err(JobStateError::UnfinishedDescendant {
                    parent: self.identity.uuid(),
                    current: self.state.clone(),
                    requested: state.clone(),
                    child: child.identity.uuid(),
                    child_state: child.state.clone(),
                }
                .into());
            }
        }

        self.state.update(self.identity.uuid(), state)?;

        if self.state.is_finished() {
            // Publish completion before notifying the shutdown observer.
            self.sensors.finish();
        }

        self.state_update.notify_one();

        if self.state.is_cancelling() {
            self.sensors.cancel();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unfinished_descendant_rejects_each_terminal_transition_atomically() {
        for child_state in [
            JobState::Created,
            JobState::Started("child".into()),
            JobState::Cancelling,
        ] {
            for outcome in [
                JobState::Success(None),
                JobState::Failed(Some("cause".into())),
                JobState::Cancelled(None),
            ] {
                let mut parent = JobEntry::new(
                    JobIdentity::new("parent", None, JobVisibility::Hidden),
                    Arc::new(Notify::new()),
                );
                let child_id = parent
                    .child("child", JobVisibility::Hidden)
                    .unwrap()
                    .identity()
                    .uuid();
                parent.find(&child_id).unwrap().state = child_state.clone();
                parent.update(JobState::Started("parent".into())).unwrap();
                if matches!(outcome, JobState::Cancelled(_)) {
                    parent.update(JobState::Cancelling).unwrap();
                }
                let previous = parent.state.clone();
                assert!(
                    matches!(parent.update(outcome.clone()), Err(E::JobState(JobStateError::UnfinishedDescendant {
                    requested, child, child_state: actual, ..
                })) if requested == outcome && child == child_id && actual == child_state)
                );
                assert_eq!(parent.state, previous);
                assert_eq!(parent.find(&child_id).unwrap().state, child_state);
                parent.find(&child_id).unwrap().state = JobState::Failed(None);
                parent.update(outcome.clone()).unwrap();
                assert_eq!(parent.state, outcome);
            }
        }
    }

    #[test]
    fn finished_child_does_not_hide_an_unfinished_grandchild() {
        let mut parent = JobEntry::new(
            JobIdentity::new("parent", None, JobVisibility::Hidden),
            Arc::new(Notify::new()),
        );
        let child = parent
            .child("child", JobVisibility::Hidden)
            .unwrap()
            .identity()
            .uuid();
        let grandchild = parent
            .find(&child)
            .unwrap()
            .child("grandchild", JobVisibility::Hidden)
            .unwrap()
            .identity()
            .uuid();
        // Simulate an existing inconsistent subtree.
        parent.find(&child).unwrap().state = JobState::Success(None);
        parent.update(JobState::Started(String::new())).unwrap();
        assert!(
            matches!(parent.update(JobState::Success(None)), Err(E::JobState(JobStateError::UnfinishedDescendant { child: id, .. })) if id == grandchild)
        );
    }

    #[test]
    fn path_filters_only_the_ancestor_chain_in_order() {
        let mut root = JobEntry::new(
            JobIdentity::new("root", None, JobVisibility::Visible),
            Arc::new(Notify::new()),
        );
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
        let mut parent = JobEntry::new(
            JobIdentity::new("parent", None, JobVisibility::default()),
            Arc::new(Notify::new()),
        );
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

#[cfg(test)]
mod snapshot_tests {
    use super::*;

    #[tokio::test]
    async fn snapshot_tracks_descendants_and_rejected_updates_do_not_notify() {
        let changed = Arc::new(Notify::new());
        let mut root = JobEntry::new(
            JobIdentity::new("root", None, JobVisibility::Hidden),
            changed.clone(),
        );
        let parent = root
            .child("parent", JobVisibility::Hidden)
            .unwrap()
            .identity()
            .uuid();
        let child = root
            .find(&parent)
            .unwrap()
            .child("child", JobVisibility::Hidden)
            .unwrap()
            .identity()
            .uuid();
        let snapshot = root.snapshot();

        root.find(&parent)
            .unwrap()
            .update(JobState::Started(String::new()))
            .unwrap();
        changed.notified().await;
        assert!(root
            .find(&parent)
            .unwrap()
            .update(JobState::Success(None))
            .is_err());
        assert!(
            tokio::time::timeout(Duration::from_millis(20), changed.notified())
                .await
                .is_err()
        );
        assert_eq!(snapshot.unfinished().len(), 2);

        root.find(&child)
            .unwrap()
            .update(JobState::Cancelling)
            .unwrap();
        changed.notified().await;
        assert_eq!(snapshot.unfinished().len(), 2);
        root.find(&child)
            .unwrap()
            .update(JobState::Cancelled(None))
            .unwrap();
        changed.notified().await;
        assert_eq!(snapshot.unfinished(), vec![parent]);
        root.find(&parent)
            .unwrap()
            .update(JobState::Success(None))
            .unwrap();
        changed.notified().await;
        assert!(snapshot.unfinished().is_empty());
    }
}
