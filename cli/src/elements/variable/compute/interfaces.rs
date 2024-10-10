use crate::elements::{compute::Operator, Compute, TokenGetter};
use std::fmt;

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dec => "-",
                Self::Div => "/",
                Self::Inc => "+",
                Self::Mlt => "*",
            }
        )
    }
}

impl fmt::Display for Compute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl TokenGetter for Compute {
    fn token(&self) -> usize {
        self.token
    }
}
