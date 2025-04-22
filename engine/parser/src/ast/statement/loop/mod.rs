#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Loop {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Loop))
    }
}

impl ReadNode<Loop> for Loop {
    fn read(parser: &mut Parser) -> Result<Option<Loop>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Loop)) {
            return Ok(None);
        }
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
        // Do not post here an error. Error will be posted (if it's not breakable loop) on semantic level.
        // This is because "crazy" proptest testing with any kind of posobility
        let self_uuid = Uuid::new_v4();
        for uuid in nodes.into_iter() {
            let Some(node) = block.find_mut_by_uuid(&uuid) else {
                return Err(LinkedErr::from(E::FailFindNode(uuid), &block));
            };
            match &mut node.get_mut_node() {
                Node::Statement(Statement::Break(node)) => {
                    node.set_target(&self_uuid);
                }
                Node::Statement(Statement::Return(node)) => node.add_target(&self_uuid),
                _ => {
                    return Err(LinkedErr::from(E::NotBreakableLoop, &block));
                }
            }
        }
        Ok(Some(Loop {
            token,
            block: Box::new(block),
            uuid: self_uuid,
        }))
    }
}
