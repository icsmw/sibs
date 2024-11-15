use lexer::{Keyword, Kind, KindId};

use crate::*;

impl ReadNode<FunctionDeclaration> for FunctionDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<FunctionDeclaration>, E> {
        let Some(sig) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Fn)) {
            return Ok(None);
        }
        let Some(name) = parser.token().cloned() else {
            return Err(E::MissedFnName);
        };
        if !matches!(name.kind, Kind::Identifier(..)) {
            return Err(E::MissedFnName);
        }
        let Some(mut inner) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Err(E::MissedFnArguments);
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
            return Err(E::MissedFnBlock);
        };
        Ok(Some(FunctionDeclaration {
            sig,
            name,
            args,
            block: Box::new(block),
        }))
    }
}
