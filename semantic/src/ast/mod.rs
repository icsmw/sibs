mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

use crate::*;

impl InferType for Node {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        match self {
            Node::ControlFlowModifier(n) => n.infer_type(scx),
            Node::Declaration(n) => n.infer_type(scx),
            Node::Expression(n) => n.infer_type(scx),
            Node::Miscellaneous(n) => n.infer_type(scx),
            Node::Root(n) => n.infer_type(scx),
            Node::Statement(n) => n.infer_type(scx),
            Node::Value(n) => n.infer_type(scx),
        }
    }
}

impl Initialize for Node {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Node::ControlFlowModifier(n) => n.initialize(scx),
            Node::Declaration(n) => n.initialize(scx),
            Node::Expression(n) => n.initialize(scx),
            Node::Miscellaneous(n) => n.initialize(scx),
            Node::Root(n) => n.initialize(scx),
            Node::Statement(n) => n.initialize(scx),
            Node::Value(n) => n.initialize(scx),
        }
    }
}

impl InferType for LinkedNode {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        fn infer_type(
            node: &Node,
            md: &Metadata,
            scx: &mut SemanticCx,
        ) -> Result<DataType, LinkedErr<E>> {
            let mut ty = node.infer_type(scx)?;
            for ppm in md.ppm.iter() {
                scx.tys.parent.set(ty);
                ty = ppm.infer_type(scx)?;
            }
            scx.tys.parent.drop();
            Ok(ty)
        }
        match infer_type(&self.node, &self.md, scx) {
            Err(mut err) => {
                if err.is_unlinked() {
                    err.relink(self);
                }
                Err(err)
            }
            Ok(ty) => Ok(ty),
        }
    }
}

impl Initialize for LinkedNode {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self.node.initialize(scx) {
            Err(mut err) => {
                if err.is_unlinked() {
                    err.relink(self);
                }
                Err(err)
            }
            Ok(ty) => Ok(ty),
        }
    }
}
