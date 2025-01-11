#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone)]
pub enum VariableCompoundType {
    Vec(Token, Box<LinkedNode>),
}

impl SrcLinking for VariableCompoundType {
    fn link(&self) -> SrcLink {
        match self {
            Self::Vec(tk, node) => src_from::tk_and_node(tk, node),
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
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

impl SrcLinking for VariableTypeDef {
    fn link(&self) -> SrcLink {
        match self {
            Self::Primitive(tk) => src_from::tk(tk),
            Self::Compound(ty) => ty.link(),
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
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
    pub r#type: VariableTypeDef,
    pub uuid: Uuid,
}

impl SrcLinking for VariableType {
    fn link(&self) -> SrcLink {
        self.r#type.link()
    }
    fn slink(&self) -> SrcLink {
        self.r#type.slink()
    }
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.r#type)
    }
}

impl From<VariableType> for Node {
    fn from(val: VariableType) -> Self {
        Node::Declaration(Declaration::VariableType(val))
    }
}
