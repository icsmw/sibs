mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

use std::{pin::Pin, sync::Arc};

use crate::*;

impl Interpret for Node {
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Node::ControlFlowModifier(n) => n.interpret(rt),
            Node::Declaration(n) => n.interpret(rt),
            Node::Expression(n) => n.interpret(rt),
            Node::Miscellaneous(n) => n.interpret(rt),
            Node::Root(n) => n.interpret(rt),
            Node::Statement(n) => n.interpret(rt),
            Node::Value(n) => n.interpret(rt),
        }
    }
}

impl Interpret for LinkedNode {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        self.node.interpret(rt).await
    }
}
