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

impl Diagnostic for VariableVariants {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        self.variants
            .last()
            .map(|node| Position::new(self.token.pos.from, node.md.link.to()))
            .unwrap_or(self.token.pos.clone())
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        self.variants.iter().collect()
    }
}

impl<'a> Lookup<'a> for VariableVariants {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.variants
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for VariableVariants {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.variants.find_mut_by_uuid(uuid)
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
