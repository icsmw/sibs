use lexer::{Kind, KindId};

use crate::*;
/// #[skip([task_args], func())]

impl ReadNode<Gatekeeper> for Gatekeeper {
    fn read(parser: &mut Parser) -> Result<Option<Gatekeeper>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Pound) {
            return Ok(None);
        }
        let Some(mut inner) = parser.between(KindId::LeftBracket, KindId::RightBracket)? else {
            return Err(E::NoGatekeeperDirective);
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
        if inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()));
        }
        if nodes.is_empty() {
            return Err(E::NoGatekeeperDirective);
        }
        Ok(Some(Gatekeeper { token, nodes }))
    }
}
