use crate::{
    elements::{incrementer::Operator, Incrementer, TokenGetter},
    reader::words,
};
use std::fmt;

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dec => words::DEC_BY,
                Self::Inc => words::INC_BY,
            }
        )
    }
}

impl fmt::Display for Incrementer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.variable, self.operator, self.right)
    }
}

impl TokenGetter for Incrementer {
    fn token(&self) -> usize {
        self.token
    }
}
