#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Variable {
    pub ident: String,
    pub token: Token,
    pub negation: Option<Token>,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for Variable {
    fn lookup(&'a self, _trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        vec![]
    }
}

impl FindMutByUuid for Variable {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        None
    }
}

impl SrcLinking for Variable {
    fn link(&self) -> SrcLink {
        if let Some(open) = self.negation.as_ref() {
            src_from::tks(open, &self.token)
        } else {
            src_from::tk(&self.token)
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.negation
                .as_ref()
                .map(|tk| format!("{tk} "))
                .unwrap_or_default(),
            self.token,
        )
    }
}

impl From<Variable> for Node {
    fn from(val: Variable) -> Self {
        Node::Expression(Expression::Variable(val))
    }
}
