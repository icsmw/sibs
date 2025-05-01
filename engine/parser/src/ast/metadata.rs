use asttree::{ExpressionId, LinkedNode, Metadata};

use crate::*;

impl ReadMetadata for Metadata {
    fn read_md_before(&mut self, parser: &Parser) -> Result<(), LinkedErr<E>> {
        self.meta = Vec::new();
        loop {
            let drop = parser.pin();
            if let Some(node) = Meta::read(parser)? {
                self.meta.push(LinkedNode::from_node(node.into()));
                continue;
            }
            drop(parser);
            if let Some(node) = Comment::read(parser)? {
                self.meta.push(LinkedNode::from_node(node.into()));
                continue;
            }
            drop(parser);
            break;
        }
        Ok(())
    }
    fn read_md_after(&mut self, parser: &Parser) -> Result<(), LinkedErr<E>> {
        self.ppm = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Expression(&[
                ExpressionId::Accessor,
                ExpressionId::Call,
            ])],
        )? {
            self.ppm.push(node);
        }
        Ok(())
    }
}
