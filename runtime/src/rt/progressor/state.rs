use console::style;
use std::fmt;

#[derive(Debug)]
pub enum ProgressState {
    Success(Option<String>),
    Failed(Option<String>),
    Progress(Option<String>, u64, u64),
    Pending(Option<String>),
    Working(Option<String>),
    Cancelled(Option<String>),
}

impl Default for ProgressState {
    fn default() -> Self {
        Self::Working(None)
    }
}

impl ProgressState {
    pub fn set_msg<S: AsRef<str>>(&mut self, msg: S) {
        match self {
            Self::Success(inner, ..)
            | Self::Failed(inner, ..)
            | Self::Progress(inner, ..)
            | Self::Pending(inner, ..)
            | Self::Working(inner, ..)
            | Self::Cancelled(inner, ..) => inner.replace(msg.as_ref().to_string()),
        };
    }
    pub fn get_msg(&self) -> Option<String> {
        match self {
            Self::Success(inner, ..)
            | Self::Failed(inner, ..)
            | Self::Progress(inner, ..)
            | Self::Pending(inner, ..)
            | Self::Working(inner, ..)
            | Self::Cancelled(inner, ..) => inner.clone(),
        }
    }
}
impl fmt::Display for ProgressState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProgressState::Success(..) => style("done".to_owned()).bold().green(),
                ProgressState::Failed(..) => style("failed".to_owned()).bold().red(),
                ProgressState::Progress(.., done, total) => style(format!(
                    "{}%",
                    (((*done as f64) / (*total as f64)) * 100.0) as u64
                ))
                .bold()
                .green(),
                ProgressState::Pending(..) => style("wait".to_owned()).bold().blue(),
                ProgressState::Working(..) => style("work".to_owned()).bold().green(),
                ProgressState::Cancelled(..) => style("cancelled".to_owned()).bold().yellow(),
            }
        )
    }
}
