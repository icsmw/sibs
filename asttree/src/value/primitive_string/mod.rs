#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PrimitiveString {
    pub inner: String,
    pub token: Token,
    pub uuid: Uuid,
}

impl fmt::Display for PrimitiveString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}
