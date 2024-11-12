#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::{Kind, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub enum VariablePrimitiveType {
    String,
    Number,
    Boolean,
}

impl fmt::Display for VariablePrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Boolean => "bool",
                Self::Number => "num",
                Self::String => "str",
            }
        )
    }
}

impl TryFrom<String> for VariablePrimitiveType {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_ref() {
            "bool" => Ok(Self::Boolean),
            "num" => Ok(Self::Number),
            "str" => Ok(Self::String),
            _ => Err(()),
        }
    }
}

impl VariablePrimitiveType {
    pub fn to_ident(&self) -> String {
        self.to_string()
    }
}

#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone)]
pub enum VariableCompoundType {
    Vec(Box<Node>),
}

impl fmt::Display for VariableCompoundType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Vec(n) => format!("Vec {} {n} {}", Kind::Less, Kind::Greater),
            }
        )
    }
}

impl VariableCompoundType {
    pub fn new<S: AsRef<str>>(ty: Node, alias: S) -> Result<Self, E> {
        if alias.as_ref() == VariableCompoundTypeId::Vec.to_string() {
            Ok(Self::Vec(Box::new(ty)))
        } else {
            Err(E::UnknownType(alias.as_ref().to_string()))
        }
    }
    pub fn is_valid_alias<S: AsRef<str>>(s: S) -> bool {
        VariableCompoundTypeId::as_vec()
            .iter()
            .any(|v| v.to_string() == s.as_ref())
    }
    pub fn to_ident(&self) -> String {
        match self {
            Self::Vec(..) => VariableCompoundTypeId::Vec.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VariableTypeDef {
    Primitive(VariablePrimitiveType),
    Compound(VariableCompoundType),
}

impl fmt::Display for VariableTypeDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Primitive(ty) => ty.to_string(),
                Self::Compound(ty) => ty.to_string(),
            }
        )
    }
}

impl VariableTypeDef {
    pub fn to_ident(&self) -> String {
        match self {
            Self::Primitive(ty) => ty.to_ident(),
            Self::Compound(ty) => ty.to_ident(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableType {
    r#type: VariableTypeDef,
    token: Token,
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.r#type)
    }
}
