use crate::{
    elements::{For, TokenGetter},
    reader::words,
};
use std::fmt;

impl fmt::Display for For {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} in {} {}",
            words::FOR,
            self.index,
            self.target,
            self.block
        )
    }
}

impl TokenGetter for For {
    fn token(&self) -> usize {
        self.token
    }
}
