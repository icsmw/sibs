#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Kind, KindId};

impl ReadNode<Closure> for Closure {
    fn read(parser: &mut Parser) -> Result<Option<Closure>, LinkedErr<E>> {
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftParen, KindId::RightParen)?
        else {
            return Ok(None);
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
        let block = Statement::try_read(parser, StatementId::Block)?
            .map(Node::Statement)
            .ok_or_else(|| E::MissedClosureBlock.link_between(&open, &close))?;
        Ok(Some(Closure {
            args,
            block: Box::new(block),
            open,
            close,
        }))
    }
}
