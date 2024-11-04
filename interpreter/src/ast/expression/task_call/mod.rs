mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct TaskCall {}

impl fmt::Display for TaskCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
