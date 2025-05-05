#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for For {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::For))
    }
}

impl ReadNode<For> for For {
    fn read(parser: &Parser) -> Result<Option<For>, LinkedErr<E>> {
        let Some(token_for) = parser.token() else {
            return Ok(None);
        };
        if !matches!(token_for.kind, Kind::Keyword(Keyword::For)) {
            return Ok(None);
        }
        let restore = parser.pin();
        let (el, index) = if let Some((mut inner, ..)) =
            parser.between(KindId::LeftParen, KindId::RightParen)?
        {
            let el = LinkedNode::try_oneof(
                &mut inner,
                &[NodeTarget::Expression(&[ExpressionId::Variable])],
            )?
            .ok_or_else(|| E::MissedElementDeclarationInFor.link_with_token(&token_for))?;
            if !inner.is_next(KindId::Comma) {
                return Err(E::MissedComma.link_by_current(&inner));
            } else {
                let _ = inner.token();
            }
            let index_ref = LinkedNode::try_oneof(
                &mut inner,
                &[NodeTarget::Expression(&[ExpressionId::Variable])],
            )?
            .ok_or_else(|| E::MissedIndexDeclarationInFor.link_by_current(&inner))?;
            if !inner.is_done() {
                return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
            };
            (el, Some(index_ref))
        } else {
            restore(parser);
            if let Some(el) =
                LinkedNode::try_oneof(parser, &[NodeTarget::Expression(&[ExpressionId::Variable])])?
            {
                (el, None)
            } else {
                return Err(E::MissedElementDeclarationInFor.link_with_token(&token_for));
            }
        };
        let Some(token_in) = parser.token() else {
            return Err(E::InvalidForSyntax.link_with_token(&token_for));
        };
        if !matches!(token_in.kind, Kind::Keyword(Keyword::In)) {
            return Err(E::MissedInKeywordInFor.link_with_token(&token_for));
        }
        let elements = LinkedNode::try_oneof(
            parser,
            &[
                NodeTarget::Value(&[ValueId::Array]),
                NodeTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::FunctionCall,
                    ExpressionId::Range,
                ]),
            ],
        )?
        .ok_or_else(|| {
            E::FailRecognizeElementsInFor(parser.to_string()).link_with_token(&token_in)
        })?;
        let mut block =
            LinkedNode::try_oneof(parser, &[NodeTarget::Statement(&[StatementId::Block])])?
                .ok_or_else(|| E::MissedBlock.link_with_token(&token_for))?;
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
            match &mut node.get_mut_node() {
                Node::Statement(Statement::Break(node)) => {
                    node.set_target(&self_uuid);
                }
                Node::Statement(Statement::Return(node)) => node.add_target(&self_uuid),
                _ => {}
            }
        }
        Ok(Some(For {
            token_for: token_for.clone(),
            token_in: token_in.clone(),
            element: Box::new(el),
            index: index.map(Box::new),
            elements: Box::new(elements),
            block: Box::new(block),
            uuid: self_uuid,
        }))
    }
}
