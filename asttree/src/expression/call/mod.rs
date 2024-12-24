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
