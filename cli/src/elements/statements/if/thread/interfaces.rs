use crate::elements::{IfThread, TokenGetter};
use std::fmt;

impl TokenGetter for IfThread {
    fn token(&self) -> usize {
        match self {
            Self::If(el, _) => el.token(),
            Self::Else(block) => block.token(),
        }
    }
}

impl fmt::Display for IfThread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::If(el, block) => format!("if {el} {block}"),
                Self::Else(block) => format!("else {block}"),
            }
        )
    }
}
