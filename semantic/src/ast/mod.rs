mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

use crate::*;

impl InferType for Node {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            Node::ControlFlowModifier(n) => n.infer_type(tcx),
            Node::Declaration(n) => n.infer_type(tcx),
            Node::Expression(n) => n.infer_type(tcx),
            Node::Miscellaneous(n) => n.infer_type(tcx),
            Node::Root(n) => n.infer_type(tcx),
            Node::Statement(n) => n.infer_type(tcx),
            Node::Value(n) => n.infer_type(tcx),
        }
    }
}

impl Initialize for Node {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        match self {
            Node::ControlFlowModifier(n) => n.initialize(tcx),
            Node::Declaration(n) => n.initialize(tcx),
            Node::Expression(n) => n.initialize(tcx),
            Node::Miscellaneous(n) => n.initialize(tcx),
            Node::Root(n) => n.initialize(tcx),
            Node::Statement(n) => n.initialize(tcx),
            Node::Value(n) => n.initialize(tcx),
        }
    }
}

impl InferType for LinkedNode {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let mut ty = self.node.infer_type(tcx)?;
        for ppm in self.md.ppm.iter() {
            tcx.set_parent_ty(ty);
            ty = ppm.infer_type(tcx)?;
        }
        tcx.drop_parent_ty();
        Ok(ty)
    }
}

impl Initialize for LinkedNode {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.node.initialize(tcx)
    }
}
