#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind, KindId};

impl ReadNode<Task> for Task {
    fn read(parser: &mut Parser) -> Result<Option<Task>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        let (sig, vis) = if matches!(token.kind, Kind::Keyword(Keyword::Private)) {
            let Some(sig) = parser.token().cloned() else {
                return Err(E::InvalidPrivateKeyUsage.link_with_token(&token));
            };
            (sig, Some(token))
        } else {
            (token, None)
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Task)) {
            return Ok(None);
        }
        let Some(name) = parser.token().cloned() else {
            return Err(E::MissedTaskName.link_with_token(&sig));
        };
        if !matches!(name.kind, Kind::Identifier(..)) {
            return Err(E::MissedTaskName.link_with_token(&sig));
        }
        let Some((mut inner, ..)) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Err(E::MissedTaskArguments.link_with_token(&sig));
        };
        let mut args = Vec::new();
        while let Some(arg) = Declaration::try_read(&mut inner, DeclarationId::ArgumentDeclaration)?
            .map(Node::Declaration)
        {
            args.push(arg);
            if let Some(tk) = inner.token() {
                if !matches!(tk.kind, Kind::Comma) {
                    return Err(E::MissedComma.link_with_token(tk));
                }
            }
        }
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
        }
        let Some(block) = Statement::try_read(parser, StatementId::Block)?.map(Node::Statement)
        else {
            return Err(E::MissedTaskBlock.link_with_token(&sig));
        };
        Ok(Some(Task {
            vis,
            sig,
            name,
            args,
            block: Box::new(block),
        }))
    }
}
