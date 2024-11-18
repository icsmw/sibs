use lexer::{Kind, KindId};

use crate::*;
/// #[skip([task_args], func())]

impl ReadNode<Gatekeeper> for Gatekeeper {
    fn read(parser: &mut Parser) -> Result<Option<Gatekeeper>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Pound) {
            return Ok(None);
        }
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftBracket, KindId::RightBracket)?
        else {
            return Err(E::NoGatekeeperDirective.link_with_token(&token));
        };
        let mut nodes = Vec::new();
        while let Some(node) = Node::try_oneof(
            &mut inner,
            &[NodeReadTarget::ControlFlowModifier(&[
                ControlFlowModifierId::Skip,
            ])],
        )? {
            nodes.push(node);
            if inner.is_next(KindId::Comma) {
                let _ = inner.token();
            }
        }
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_from_current(&inner));
        }
        if nodes.is_empty() {
            return Err(E::NoGatekeeperDirective.link_with_token(&token));
        }
        Ok(Some(Gatekeeper {
            token,
            nodes,
            open,
            close,
        }))
    }
}
