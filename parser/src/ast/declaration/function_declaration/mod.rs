#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind, KindId};

impl ReadNode<FunctionDeclaration> for FunctionDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<FunctionDeclaration>, LinkedErr<E>> {
        let Some(sig) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Fn)) {
            return Ok(None);
        }
        let name = parser
            .token()
            .cloned()
            .ok_or_else(|| E::MissedFnName.link_with_token(&sig))?;
        if !matches!(name.kind, Kind::Identifier(..)) {
            return Err(E::MissedFnName.link_with_token(&sig));
        }
        let (mut inner, _, close_tk) = parser
            .between(KindId::LeftParen, KindId::RightParen)?
            .ok_or_else(|| E::MissedFnArguments.link_between(&sig, &name))?;
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
            .ok_or_else(|| E::MissedFnBlock.link_between(&sig, &close_tk))?;
        Ok(Some(FunctionDeclaration {
            sig,
            name,
            args,
            block: Box::new(block),
        }))
    }
}
