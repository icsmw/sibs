use crate::{
    elements::{conditions::Cmp, Comparing, TokenGetter},
    reader::words,
};
use std::fmt;

impl fmt::Display for Cmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Equal => words::CMP_TRUE,
                Self::NotEqual => words::CMP_FALSE,
                Self::LeftBig => words::CMP_LBIG,
                Self::RightBig => words::CMP_RBIG,
                Self::LeftBigInc => words::CMP_LBIG_INC,
                Self::RightBigInc => words::CMP_RBIG_INC,
            }
        )
    }
}

impl fmt::Display for Comparing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.cmp, self.right)
    }
}

impl TokenGetter for Comparing {
    fn token(&self) -> usize {
        self.token
    }
}
