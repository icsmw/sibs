#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Closure {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::VerticalBar)
    }
}

impl ReadNode<Closure> for Closure {
    fn read(parser: &mut Parser) -> Result<Option<Closure>, LinkedErr<E>> {
        let Some(open) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(open.kind, Kind::VerticalBar) {
            return Ok(None);
        }
        let mut args = Vec::new();
        let mut close = None;
        while let Some(arg) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Declaration(&[
                DeclarationId::ArgumentDeclaration,
            ])],
        )? {
            args.push(arg);
            if let Some(tk) = parser.token() {
                if matches!(tk.kind, Kind::VerticalBar) {
                    close = Some(tk.clone());
                    break;
                }
                if !matches!(tk.kind, Kind::Comma) {
                    return Err(E::MissedComma.link_with_token(tk));
                }
            }
        }
        let Some(close) = close else {
            return Err(E::MissedClosingBar.link_with_token(&open));
        };
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
