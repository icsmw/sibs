#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Call {
    pub token: Token,
    pub node: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl Call {
    pub fn get_name(&self) -> Option<String> {
        let Node::Expression(Expression::FunctionCall(fn_call)) = &self.node.node else {
            return None;
        };
        Some(fn_call.get_name())
    }
    pub fn get_fn(&self) -> Option<&FunctionCall> {
        if let Node::Expression(Expression::FunctionCall(fn_call)) = &self.node.node {
            Some(fn_call)
        } else {
            None
        }
    }
}

impl<'a> Lookup<'a> for Call {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node.lookup_inner(self.uuid, trgs)
    }
}

impl SrcLinking for Call {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.token, &self.node)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.token, self.node)
    }
}

impl From<Call> for Node {
    fn from(val: Call) -> Self {
        Node::Expression(Expression::Call(val))
    }
}
