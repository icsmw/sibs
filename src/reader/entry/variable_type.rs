use crate::reader::{
    chars,
    entry::{Reader, Reading},
    E,
};
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
    pub index: usize,
}

impl Reading<VariableType> for VariableType {
    fn read(reader: &mut Reader) -> Result<Option<VariableType>, E> {
        if reader.move_to_char(&[chars::TYPE_OPEN])?.is_some() {
            if let Some((word, _, index)) = reader.read_word(&[chars::TYPE_CLOSE], true)? {
                Ok(Some(VariableType::new(word, index)?))
            } else {
                Err(E::NotClosedTypeDeclaration)
            }
        } else {
            Ok(None)
        }
    }
}

impl VariableType {
    pub fn new(var_type: String, index: usize) -> Result<Self, E> {
        if Types::String.to_string() == var_type {
            return Ok(Self {
                var_type: Types::String,
                index,
            });
        }
        if Types::Bool.to_string() == var_type {
            return Ok(Self {
                var_type: Types::Bool,
                index,
            });
        }
        if Types::Number.to_string() == var_type {
            return Ok(Self {
                var_type: Types::Number,
                index,
            });
        }
        Err(E::UnknownVariableType(var_type))
    }
}
