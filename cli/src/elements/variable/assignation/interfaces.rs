use crate::{
    elements::{TokenGetter, VariableAssignation},
    reader::words,
};
use std::fmt;

impl fmt::Display for VariableAssignation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{} = {}",
            if self.global {
                format!("{} ", words::GLOBAL_VAR)
            } else {
                String::new()
            },
            self.variable,
            self.assignation
        )
    }
}

impl TokenGetter for VariableAssignation {
    fn token(&self) -> usize {
        self.token
    }
}
