use crate::*;

pub trait Interest {
    fn intrested(token: &Token) -> bool;
}
