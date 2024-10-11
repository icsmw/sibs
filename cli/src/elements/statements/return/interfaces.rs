use crate::{
    elements::{Return, TokenGetter},
    reader::words,
};
use std::fmt;

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            words::RETURN,
            if let Some(el) = self.output.as_ref() {
                format!(" {el}")
            } else {
                String::new()
            }
        )
    }
}

impl TokenGetter for Return {
    fn token(&self) -> usize {
        self.token
    }
}
