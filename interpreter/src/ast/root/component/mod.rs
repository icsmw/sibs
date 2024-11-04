mod read;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Component {}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
