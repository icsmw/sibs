#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for While {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::While))
    }
}

impl ReadNode<While> for While {
    fn read(parser: &mut Parser) -> Result<Option<While>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::While)) {
            return Ok(None);
        }
        let comparison = LinkedNode::try_oneof(
            parser,
            &[
                NodeTarget::Expression(&[ExpressionId::ComparisonSeq, ExpressionId::Variable]),
                NodeTarget::Value(&[ValueId::Boolean]),
            ],
        )?
        .ok_or_else(|| E::MissedComparisonInWhile.link_with_token(&token))?;
        let mut block =
            LinkedNode::try_oneof(parser, &[NodeTarget::Statement(&[StatementId::Block])])?
                .ok_or_else(|| E::MissedBlock.link_with_token(&token))?;
        let nodes = block
            .lookup(&[NodeTarget::Statement(&[
                StatementId::Break,
                StatementId::Return,
            ])])
            .into_iter()
            .map(|n| *(n.node.uuid()))
            .collect::<Vec<Uuid>>();
        let self_uuid = Uuid::new_v4();
        for uuid in nodes.into_iter() {
            let Some(node) = block.find_mut_by_uuid(&uuid) else {
                return Err(LinkedErr::from(E::FailFindNode(uuid), &block));
            };
            match &mut node.node {
                Node::Statement(Statement::Break(node)) => {
                    if node.target.is_none() {
                        node.target = Some(self_uuid)
                    }
                }
                Node::Statement(Statement::Return(node)) => node.add_target(&self_uuid),
                _ => {}
            }
        }
        Ok(Some(While {
            token,
            comparison: Box::new(comparison),
            block: Box::new(block),
            uuid: self_uuid,
        }))
    }
}
