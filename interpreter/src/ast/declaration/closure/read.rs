use lexer::{Kind, KindId};

use crate::*;

impl ReadNode<Closure> for Closure {
    fn read(parser: &mut Parser) -> Result<Option<Closure>, E> {
        let Some(mut inner) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
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
            return Err(E::MissedClosureBlock);
        };
        Ok(Some(Closure {
            args,
            block: Box::new(block),
        }))
    }
}
