#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableVariants {
    pub variants: Vec<LinkedNode>,
    pub token: Token,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for VariableVariants {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.variants
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl SrcLinking for VariableVariants {
    fn link(&self) -> SrcLink {
        if let Some(n) = self.variants.last() {
            src_from::tk_and_node(&self.token, n)
        } else {
            src_from::tk(&self.token)
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for VariableVariants {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.token,
            self.variants
                .iter()
                .map(|ty| ty.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::VerticalBar))
        )
    }
}

impl From<VariableVariants> for Node {
    fn from(val: VariableVariants) -> Self {
        Node::Declaration(Declaration::VariableVariants(val))
    }
}
