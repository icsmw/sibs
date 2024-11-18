use crate::*;
use lexer::{Keyword, Kind, KindId};

impl ReadNode<Component> for Component {
    fn read(parser: &mut Parser) -> Result<Option<Component>, LinkedErr<E>> {
        let Some(sig) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Component)) {
            return Ok(None);
        }
        let Some(name) = parser.token().cloned() else {
            return Err(E::MissedComponentName.link_with_token(&sig));
        };
        if !matches!(name.kind, Kind::Identifier(..)) {
            return Err(E::MissedComponentName.link_with_token(&sig));
        }
        let Some((inner, ..)) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Err(E::MissedComponentCWD.link_with_token(&sig));
        };
        let path = inner.to_string();
        let Some((mut inner, open_bl, close_bl)) =
            parser.between(KindId::LeftBrace, KindId::RightBrace)?
        else {
            return Err(E::MissedComponentBlock.link_with_token(&sig));
        };
        let mut nodes = Vec::new();
        loop {
            'semicolons: loop {
                if inner.is_next(KindId::Semicolon) {
                    let _ = inner.token();
                } else {
                    break 'semicolons;
                }
            }
            let Some(node) = Node::try_oneof(
                &mut inner,
                &[
                    NodeReadTarget::ControlFlowModifier(&[ControlFlowModifierId::Gatekeeper]),
                    NodeReadTarget::Root(&[RootId::Task]),
                    NodeReadTarget::Miscellaneous(&[
                        MiscellaneousId::Comment,
                        MiscellaneousId::Meta,
                    ]),
                ],
            )?
            else {
                break;
            };
            nodes.push(node);
        }
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_from_current(&inner));
        }
        if nodes.is_empty() {
            return Err(E::NoTasksInComponent.link_with_token(&sig));
        }
        Ok(Some(Component {
            sig,
            name,
            path,
            nodes,
            open_bl,
            close_bl,
        }))
    }
}
