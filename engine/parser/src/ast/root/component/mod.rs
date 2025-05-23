#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Component {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Component))
    }
}

impl ReadNode<Component> for Component {
    fn read(parser: &Parser) -> Result<Option<Component>, LinkedErr<E>> {
        let Some(sig) = parser.token() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Component)) {
            return Ok(None);
        }
        let name = parser
            .token()
            .ok_or_else(|| E::MissedComponentName.link_with_token(&sig))?;
        if !matches!(name.kind, Kind::Identifier(..)) {
            return Err(E::MissedComponentName.link_with_token(&sig));
        }
        let (inner, ..) = parser
            .between(KindId::LeftParen, KindId::RightParen)?
            .ok_or_else(|| E::MissedComponentCWD.link_with_token(&sig))?;
        let path = inner.to_string().trim().to_owned();
        let (mut inner, open_bl, close_bl) = parser
            .between(KindId::LeftBrace, KindId::RightBrace)?
            .ok_or_else(|| E::MissedComponentBlock.link_with_token(&sig))?;
        let mut nodes = Vec::new();
        loop {
            'semicolons: loop {
                if inner.is_next(KindId::Semicolon) {
                    let _ = inner.token();
                } else {
                    break 'semicolons;
                }
            }
            let Some(node) =
                LinkedNode::try_oneof(&mut inner, &[NodeTarget::Root(&[RootId::Task])])?
            else {
                break;
            };
            nodes.push(node);
        }
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
        }
        if nodes.is_empty() {
            return Err(E::NoTasksInComponent.link_with_token(&sig));
        }
        Ok(Some(Component {
            sig: sig.clone(),
            name: name.clone(),
            path,
            nodes,
            open_bl: open_bl.clone(),
            close_bl: close_bl.clone(),
            uuid: Uuid::new_v4(),
        }))
    }
}
