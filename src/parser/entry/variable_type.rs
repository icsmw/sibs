use crate::parser::{
    chars,
    entry::{Reader, Reading},
    E,
};
use std::fmt;
use uuid::Uuid;

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
    pub uuid: Uuid,
}

impl Reading<VariableType> for VariableType {
    fn read(reader: &mut Reader) -> Result<Option<VariableType>, E> {
        if reader.move_to_char(chars::TYPE_OPEN)? {
            if let Some((word, _, uuid)) = reader.read_word(&[chars::TYPE_CLOSE], true)? {
                Ok(Some(VariableType::new(word, uuid)?))
            } else {
                Err(E::NotClosedTypeDeclaration)
            }
        } else {
            Ok(None)
        }
    }
}

impl VariableType {
    pub fn new(var_type: String, uuid: Uuid) -> Result<Self, E> {
        if Types::String.to_string() == var_type {
            return Ok(Self {
                var_type: Types::String,
                uuid,
            });
        }
        if Types::Bool.to_string() == var_type {
            return Ok(Self {
                var_type: Types::Bool,
                uuid,
            });
        }
        if Types::Number.to_string() == var_type {
            return Ok(Self {
                var_type: Types::Number,
                uuid,
            });
        }
        Err(E::UnknownVariableType(var_type))
    }
}
