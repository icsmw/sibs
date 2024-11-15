use lexer::{Keyword, Kind, KindId};

use crate::*;

impl ReadNode<Task> for Task {
    fn read(parser: &mut Parser) -> Result<Option<Task>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        let (sig, vis) = if matches!(token.kind, Kind::Keyword(Keyword::Private)) {
            let Some(sig) = parser.token().cloned() else {
                return Err(E::InvalidPrivateKeyUsage);
            };
            (sig, Some(token))
        } else {
            (token, None)
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Task)) {
            return Ok(None);
        }
        let Some(name) = parser.token().cloned() else {
            return Err(E::MissedTaskName);
        };
        if !matches!(name.kind, Kind::Identifier(..)) {
            return Err(E::MissedTaskName);
        }
        let Some(mut inner) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Err(E::MissedTaskArguments);
        };
        let mut args = Vec::new();
        while let Some(arg) = Declaration::try_read(&mut inner, DeclarationId::ArgumentDeclaration)?
            .map(Node::Declaration)
        {
            args.push(arg);
            if let Some(tk) = inner.token() {
                if !matches!(tk.kind, Kind::Comma) {
                    return Err(E::MissedComma);
                }
            }
        }
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()));
        }
        let Some(block) = Statement::try_read(parser, StatementId::Block)?.map(Node::Statement)
        else {
            return Err(E::MissedTaskBlock);
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
