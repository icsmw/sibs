#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for ClosureDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Colon)
    }
}

impl ReadNode<ClosureDeclaration> for ClosureDeclaration {
    fn read(parser: &Parser) -> Result<Option<ClosureDeclaration>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Colon) {
            return Ok(None);
        }
        let Some(open) = parser.token() else {
            return Ok(None);
        };
        if !matches!(open.kind, Kind::VerticalBar) {
            return Ok(None);
        }
        let mut args = Vec::new();
        let mut close = None;
        while let Some(arg) = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Declaration(&[
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
                    return Err(E::MissedComma.link_with_token(&tk));
                }
            }
        }
        let Some(close) = close else {
            return Err(E::MissedClosingBar.link_with_token(&open));
        };
        let ty = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Declaration(&[
                DeclarationId::VariableTypeDeclaration,
            ])],
        )?
        .ok_or_else(|| E::MissedClosureReturnType.link_between(&open, &close))?;
        Ok(Some(ClosureDeclaration {
            token: token.clone(),
            args,
            ty: Box::new(ty),
            open: open.clone(),
            close,
            uuid: Uuid::new_v4(),
        }))
    }
}
