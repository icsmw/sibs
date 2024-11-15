use crate::*;
use lexer::{Keyword, Kind, KindId};

impl ReadNode<Component> for Component {
    fn read(parser: &mut Parser) -> Result<Option<Component>, E> {
        let Some(sig) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Component)) {
            return Ok(None);
        }
        let Some(name) = parser.token().cloned() else {
            return Err(E::MissedComponentName);
        };
        if !matches!(name.kind, Kind::Identifier(..)) {
            return Err(E::MissedComponentName);
        }
        let Some(inner) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Err(E::MissedComponentCWD);
        };
        let path = inner.to_string();
        let Some(mut inner) = parser.between(KindId::LeftBrace, KindId::RightBrace)? else {
            return Err(E::MissedComponentBlock);
        };
        let mut tasks = Vec::new();
        loop {
            'semicolons: loop {
                if inner.is_next(KindId::Semicolon) {
                    let _ = inner.token();
                } else {
                    break 'semicolons;
                }
            }
            let Some(task) = Root::try_read(&mut inner, RootId::Task)?.map(Node::Root) else {
                break;
            };
            tasks.push(task);
        }
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()));
        }
        if tasks.is_empty() {
            return Err(E::NoTasksInComponent);
        }
        Ok(Some(Component {
            sig,
            name,
            path,
            tasks,
        }))
    }
}
