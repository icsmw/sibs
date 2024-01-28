use crate::{
    inf::term::{self},
    reader::{
        chars,
        entry::{Reader, Reading},
        E,
    },
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
    pub token: usize,
}

impl VariableType {
    pub fn parse(&self, value: String) -> Option<String> {
        match &self.var_type {
            Types::String => Some(value),
            Types::Number => value.parse::<isize>().ok().map(|_| value),
            Types::Bool => value.parse::<bool>().ok().map(|_| value),
        }
    }
}

impl Reading<VariableType> for VariableType {
    fn read(reader: &mut Reader) -> Result<Option<VariableType>, E> {
        if reader.move_to().char(&[&chars::TYPE_OPEN]).is_some() {
            if let Some((word, _char)) = reader.until().char(&[&chars::TYPE_CLOSE]) {
                reader.move_to().next();
                Ok(Some(VariableType::new(word, reader.token()?.id)?))
            } else {
                Err(E::NotClosedTypeDeclaration)
            }
        } else {
            Ok(None)
        }
    }
}

impl VariableType {
    pub fn new(var_type: String, token: usize) -> Result<Self, E> {
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
        Err(E::UnknownVariableType(var_type))
    }
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.var_type)
    }
}

impl term::Display for VariableType {
    fn to_string(&self) -> String {
        format!("{{{}}}", self.var_type)
    }
}
