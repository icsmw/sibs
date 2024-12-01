use asttree::{ExpressionId, LinkedNode, Metadata, MiscellaneousId};

use crate::*;

impl ReadMetadata for Metadata {
    fn read_md_before(&mut self, parser: &mut Parser) -> Result<(), LinkedErr<E>> {
        self.meta = Vec::new();
        while let Some(node) = read_and_resolve_nodes(
            parser,
            &[NodeReadTarget::Miscellaneous(&[
                MiscellaneousId::Comment,
                MiscellaneousId::Meta,
            ])],
        )? {
            self.meta.push(LinkedNode::from_node(node));
        }
        Ok(())
    }
    fn read_md_after(&mut self, parser: &mut Parser) -> Result<(), LinkedErr<E>> {
        self.ppm = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Expression(&[
                ExpressionId::Accessor,
                ExpressionId::Call,
            ])],
        )? {
            self.ppm.push(node);
        }
        Ok(())
    }
}
