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
        while let Some(arg) = LinkedNode::try_oneof(
            &mut inner,
            &[NodeReadTarget::Declaration(&[
                DeclarationId::ArgumentDeclaration,
            ])],
        )? {
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
        let block =
            LinkedNode::try_oneof(parser, &[NodeReadTarget::Statement(&[StatementId::Block])])?
                .ok_or_else(|| E::MissedClosureBlock.link_between(&open, &close))?;
        Ok(Some(Closure {
            args,
            block: Box::new(block),
            open,
            close,
            uuid: Uuid::new_v4(),
        }))
    }
}
