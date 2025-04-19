#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Gatekeeper {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Pound)
    }
}

impl ReadNode<Gatekeeper> for Gatekeeper {
    fn read(parser: &mut Parser) -> Result<Option<Gatekeeper>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Pound) {
            return Ok(None);
        }
        let (mut inner, open, close) =
            parser
                .between(KindId::LeftBracket, KindId::RightBracket)?
                .ok_or_else(|| E::NoGatekeeperDirective.link_with_token(&token))?;
        let mut nodes = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            &mut inner,
            &[NodeTarget::ControlFlowModifier(&[
                ControlFlowModifierId::Skip,
            ])],
        )? {
            nodes.push(node);
            if inner.is_next(KindId::Comma) {
                let _ = inner.token();
            }
        }
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
        }
        if nodes.is_empty() {
            return Err(E::NoGatekeeperDirective.link_with_token(&token));
        }
        Ok(Some(Gatekeeper {
            token,
            nodes,
            open,
            close,
            uuid: Uuid::new_v4(),
        }))
    }
}
