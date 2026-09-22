use std::fmt;

mod error;
pub use error::*;
use uuid::Uuid;

use crate::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum JobState {
    #[default]
    Created,
    Started(String),
    // Cancellation has begun; execution and cleanup may still be running.
    Cancelling,
    // Task cancelled
    Cancelled(Option<String>),
    //Timeouted
    Success(Option<String>),
    Failed(Option<String>),
}

impl JobState {
    pub(super) fn is_cancelling(&self) -> bool {
        matches!(self, JobState::Cancelling)
    }
    pub(super) fn is_locked(&self) -> bool {
        !matches!(self, JobState::Created | JobState::Started(_))
    }
    pub(super) fn lock(&mut self, uuid: Uuid) -> Result<(), JobStateError> {
        self.update(uuid, JobState::Cancelling)
    }
    pub fn is_finished(&self) -> bool {
        match self {
            Self::Success(_) | Self::Failed(_) | Self::Cancelled(_) => true,
            Self::Created | Self::Started(_) | Self::Cancelling => false,
        }
    }

    pub fn would_update(&self, uuid: Uuid, other: &JobState) -> Result<(), JobStateError> {
        if self == other {
            return Err(JobStateError::JobStateAlreadySet(uuid, other.clone()));
        }
        if other == &Self::Created {
            return Err(JobStateError::CannotSetPending(uuid));
        }
        if !match self {
            Self::Created => {
                matches!(other, Self::Started(..) | Self::Cancelling)
            }
            Self::Started(_) => {
                matches!(
                    other,
                    Self::Success(_) | Self::Failed(..) | Self::Cancelling
                )
            }
            Self::Cancelling => {
                matches!(
                    other,
                    Self::Cancelled(..) | Self::Success(..) | Self::Failed(..)
                )
            }
            Self::Cancelled(_) | Self::Success(_) | Self::Failed(_) => false,
        } {
            Err(JobStateError::InvalidOrder(
                uuid,
                self.clone(),
                other.clone(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn update(&mut self, uuid: Uuid, other: JobState) -> Result<(), JobStateError> {
        self.would_update(uuid, &other)?;
        *self = other;
        Ok(())
    }
    pub fn msg(&self) -> Option<String> {
        match self {
            Self::Created | Self::Cancelling => None,
            Self::Started(msg) => Some(msg.to_string()),
            Self::Cancelled(msg) | Self::Success(msg) | Self::Failed(msg) => msg.clone(),
        }
    }
}

impl fmt::Display for JobState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Created => "Created".to_string(),
                Self::Started(msg) => format!("Started({msg})"),
                Self::Cancelling => "Cancelling".to_string(),
                Self::Cancelled(msg) =>
                    format!("Cancelled({})", msg.as_ref().unwrap_or(&String::new())),
                Self::Success(msg) =>
                    format!("Success({})", msg.as_ref().unwrap_or(&String::new())),
                Self::Failed(msg) => format!("Failed({})", msg.as_ref().unwrap_or(&String::new())),
            }
        )
    }
}

impl From<&JobState> for scheme::EventTy {
    fn from(state: &JobState) -> Self {
        match state {
            JobState::Created => scheme::EventTy::Created,
            JobState::Started(_) => scheme::EventTy::Started,
            JobState::Cancelling => scheme::EventTy::Cancelling,
            JobState::Cancelled(_) => scheme::EventTy::Cancelled,
            JobState::Success(_) => scheme::EventTy::Success,
            JobState::Failed(_) => scheme::EventTy::Failed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_can_start_or_begin_cancellation() {
        let uuid = Uuid::new_v4();
        for next in [JobState::Started("work".into()), JobState::Cancelling] {
            let mut state = JobState::Created;
            state.update(uuid, next.clone()).expect("valid transition");
            assert_eq!(state, next);
        }
    }

    #[test]
    fn cancellation_cleanup_can_report_a_failure() {
        let uuid = Uuid::new_v4();
        let mut state = JobState::Cancelling;
        state
            .update(uuid, JobState::Failed(Some("cleanup failed".into())))
            .unwrap();
        assert_eq!(state, JobState::Failed(Some("cleanup failed".into())));
    }

    #[test]
    fn returning_to_created_preserves_the_current_state() {
        let uuid = Uuid::new_v4();
        let mut state = JobState::Started("work".into());
        let previous = state.clone();
        assert!(matches!(
            state.update(uuid, JobState::Created),
            Err(JobStateError::CannotSetPending(id)) if id == uuid
        ));
        assert_eq!(state, previous);
    }

    #[test]
    fn created_cannot_finish_without_starting() {
        let uuid = Uuid::new_v4();
        for next in [
            JobState::Success(None),
            JobState::Failed(None),
            JobState::Cancelled(None),
        ] {
            let mut state = JobState::Created;
            assert!(matches!(
                state.update(uuid, next),
                Err(JobStateError::InvalidOrder(..))
            ));
            assert_eq!(state, JobState::Created);
        }
    }
}
