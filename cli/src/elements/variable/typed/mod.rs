mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

use crate::{error::LinkedErr, reader::E};

#[derive(Debug, Clone)]
pub enum Types {
    String,
    Number,
    Bool,
}

#[derive(Debug, Clone)]
pub struct VariableType {
    pub var_type: Types,
    pub token: usize,
}

impl VariableType {
    pub fn new(var_type: String, token: usize) -> Result<Self, LinkedErr<E>> {
        if Types::String.to_string() == var_type {
            return Ok(Self {
                var_type: Types::String,
                token,
            });
        }
        if Types::Bool.to_string() == var_type {
            return Ok(Self {
                var_type: Types::Bool,
                token,
            });
        }
        if Types::Number.to_string() == var_type {
            return Ok(Self {
                var_type: Types::Number,
                token,
            });
        }
        Err(E::UnknownVariableType(var_type).linked(&token))
    }
}
