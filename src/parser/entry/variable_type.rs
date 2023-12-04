use crate::parser::E;
use std::fmt;

#[derive(Debug)]
pub enum Types {
    String,
    Number,
    Bool,
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String => "string",
                Self::Number => "number",
                Self::Bool => "bool",
            }
        )
    }
}

#[derive(Debug)]
pub struct VariableType {
    pub var_type: Types,
}

impl VariableType {
    pub fn new(var_type: String) -> Result<Self, E> {
        if Types::String.to_string() == var_type {
            return Ok(Self {
                var_type: Types::String,
            });
        }
        if Types::Bool.to_string() == var_type {
            return Ok(Self {
                var_type: Types::Bool,
            });
        }
        if Types::Number.to_string() == var_type {
            return Ok(Self {
                var_type: Types::Number,
            });
        }
        Err(E::UnknownVariableType(var_type))
    }
}
