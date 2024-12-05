use crate::*;

pub(crate) trait Interest {
    fn intrested(&self, token: &Token) -> bool;
}
