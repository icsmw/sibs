mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

use crate::*;

impl InferType for Node {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
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

impl Finalization for Node {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Node::ControlFlowModifier(n) => n.finalize(scx),
            Node::Declaration(n) => n.finalize(scx),
            Node::Expression(n) => n.finalize(scx),
            Node::Miscellaneous(n) => n.finalize(scx),
            Node::Root(n) => n.finalize(scx),
            Node::Statement(n) => n.finalize(scx),
            Node::Value(n) => n.finalize(scx),
        }
    }
}

impl SemanticTokensGetter for Node {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        match self {
            Node::ControlFlowModifier(n) => n.get_semantic_tokens(stcx),
            Node::Declaration(n) => n.get_semantic_tokens(stcx),
            Node::Expression(n) => n.get_semantic_tokens(stcx),
            Node::Miscellaneous(n) => n.get_semantic_tokens(stcx),
            Node::Root(n) => n.get_semantic_tokens(stcx),
            Node::Statement(n) => n.get_semantic_tokens(stcx),
            Node::Value(n) => n.get_semantic_tokens(stcx),
        }
    }
}

impl InferType for LinkedNode {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        fn infer_type(
            node: &Node,
            md: &Metadata,
            scx: &mut SemanticCx,
        ) -> Result<Ty, LinkedErr<E>> {
            let mut ty = node.infer_type(scx)?;
            for ppm in md.ppm.iter() {
                scx.tys
                    .get_mut()
                    .map_err(|err| LinkedErr::from(err.into(), ppm))?
                    .parent
                    .set(ppm.uuid(), ty);
                ty = ppm.infer_type(scx)?;
            }
            Ok(ty)
        }
        infer_type(self.get_node(), self.get_md(), scx)
    }
}

impl Initialize for LinkedNode {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let Err(err) = self.get_node().initialize(scx) {
            if scx.is_resilience() {
                scx.errs.add(err);
            } else {
                return Err(err);
            }
        }
        Ok(())
    }
}

impl Finalization for LinkedNode {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        fn initialize_and_finalize(
            node: &Node,
            md: &Metadata,
            scx: &mut SemanticCx,
        ) -> Result<(), LinkedErr<E>> {
            if md.ppm.is_empty() {
                return Ok(());
            }
            let mut ty = node.infer_type(scx)?;
            for ppm in md.ppm.iter() {
                scx.tys
                    .get_mut()
                    .map_err(|err| LinkedErr::from(err.into(), ppm))?
                    .parent
                    .set(ppm.uuid(), ty);
                ppm.initialize(scx)?;
                ppm.finalize(scx)?;
                scx.register(ppm.uuid(), &Ty::Indeterminate);
                ty = ppm.infer_type(scx)?;
                scx.register(ppm.uuid(), &ty);
            }
            Ok(())
        }
        if let Err(err) = initialize_and_finalize(self.get_node(), self.get_md(), scx) {
            if scx.is_resilience() {
                scx.errs.add(err);
            } else {
                return Err(err);
            }
        }
        if let Err(err) = self.get_node().finalize(scx) {
            if scx.is_resilience() {
                scx.errs.add(err);
            } else {
                return Err(err);
            }
        }
        if !matches!(
            self.get_node(),
            Node::Expression(Expression::Accessor(..)) | Node::Expression(Expression::Call(..))
        ) {
            scx.by_node(self.get_node())?;
        }
        Ok(())
    }
}

impl SemanticTokensGetter for LinkedNode {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = self.get_node().get_semantic_tokens(stcx);
        tokens.extend(
            self.get_md()
                .ppm
                .iter()
                .flat_map(|n| n.get_semantic_tokens(stcx)),
        );
        tokens.extend(
            self.get_md()
                .meta
                .iter()
                .flat_map(|n| n.get_semantic_tokens(stcx)),
        );
        tokens
    }
}
