mod diagnostic;

use asttree::{Anchor, LinkedNode};
pub use diagnostic::*;
use diagnostics::Diagnostics;
use semantic::SemanticCx;

#[derive(Debug, Default)]
pub struct InterContext {
    anchor: Option<LinkedNode>,
    scx: Option<SemanticCx>,
    diagnostics: Option<Diagnostics<DiagnosticError>>,
}

impl InterContext {
    pub fn is_empty(&self) -> bool {
        self.anchor.is_none() && self.scx.is_none() && self.diagnostics.is_none()
    }
    pub fn set_anchor(&mut self, anchor: LinkedNode) {
        self.anchor = Some(anchor);
    }
    pub fn set_semantic_cx(&mut self, scx: SemanticCx) {
        self.scx = Some(scx)
    }
    pub fn set_diagnostics(&mut self, diagnostics: Diagnostics<DiagnosticError>) {
        self.diagnostics = Some(diagnostics)
    }

    pub fn get_anchor(&self) -> Option<&LinkedNode> {
        self.anchor.as_ref()
    }
    pub fn get_anchor_inner(&self) -> Option<&Anchor> {
        self.anchor
            .as_ref()
            .and_then(|node| node.extract::<Anchor>())
    }
    pub fn get_semantic_cx(&self) -> Option<&SemanticCx> {
        self.scx.as_ref()
    }
    pub fn get_diagnostics(&self) -> Option<&Diagnostics<DiagnosticError>> {
        self.diagnostics.as_ref()
    }
    pub fn get_diagnostics_mut(&mut self) -> Option<&mut Diagnostics<DiagnosticError>> {
        self.diagnostics.as_mut()
    }

    pub(crate) fn get_script_ctx(
        &mut self,
    ) -> (
        Option<&mut LinkedNode>,
        Option<&mut SemanticCx>,
        Option<&mut Diagnostics<DiagnosticError>>,
    ) {
        (
            self.anchor.as_mut(),
            self.scx.as_mut(),
            self.diagnostics.as_mut(),
        )
    }
    pub(crate) fn get_executor_ctx(&mut self) -> (Option<&mut LinkedNode>, Option<SemanticCx>) {
        (self.anchor.as_mut(), self.scx.take())
    }
}
