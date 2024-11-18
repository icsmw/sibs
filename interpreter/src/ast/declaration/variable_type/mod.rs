mod link;
#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::{Kind, Token};
use std::fmt;

#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone)]
pub enum VariableCompoundType {
    Vec(Token, Box<Node>),
}

impl fmt::Display for VariableCompoundType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Vec(t, n) => format!("{t} {} {n} {}", Kind::Less, Kind::Greater),
            }
        )
    }
}

impl VariableCompoundType {
    pub fn to_ident(&self) -> String {
        match self {
            Self::Vec(t, _) => t.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VariableTypeDef {
    Primitive(Token),
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
            Self::Primitive(ty) => ty.to_string(),
            Self::Compound(ty) => ty.to_ident(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableType {
    r#type: VariableTypeDef,
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.r#type)
    }
}
