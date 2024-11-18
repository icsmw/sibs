use lexer::SrcLink;

use crate::*;

impl From<&VariableCompoundType> for SrcLink {
    fn from(node: &VariableCompoundType) -> Self {
        match node {
            VariableCompoundType::Vec(tk, node) => {
                let ty: SrcLink = node.into();
                SrcLink {
                    from: tk.pos.from,
                    to: ty.to,
                    src: ty.src,
                }
            }
        }
    }
}

impl From<&VariableTypeDef> for SrcLink {
    fn from(node: &VariableTypeDef) -> Self {
        match node {
            VariableTypeDef::Primitive(tk) => tk.into(),
            VariableTypeDef::Compound(ty) => ty.into(),
        }
    }
}

impl From<&VariableType> for SrcLink {
    fn from(node: &VariableType) -> Self {
        (&node.r#type).into()
    }
}
