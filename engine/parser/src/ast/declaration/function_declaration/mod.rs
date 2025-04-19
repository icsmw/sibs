#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for FunctionDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Fn))
    }
}

impl ReadNode<FunctionDeclaration> for FunctionDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<FunctionDeclaration>, LinkedErr<E>> {
        let Some(sig) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Fn)) {
            return Ok(None);
        }
        let restore = parser.pin();
        let Some(next) = parser.token() else {
            return Err(LinkedErr::token(E::MissedFnName, &sig));
        };
        if matches!(next.kind, Kind::Keyword(..)) {
            return Err(LinkedErr::token(E::KeywordUsing, next));
        }
        restore(parser);
        let name = parser
            .token()
            .cloned()
            .ok_or_else(|| E::MissedFnName.link_with_token(&sig))?;
        if !matches!(name.kind, Kind::Identifier(..)) {
            return Err(E::MissedFnName.link_with_token(&sig));
        }
        let (mut inner, open_tk, close_tk) = parser
            .between(KindId::LeftParen, KindId::RightParen)?
            .ok_or_else(|| E::MissedFnArguments.link_between(&sig, &name))?;
        let mut args = Vec::new();
        while let Some(arg) = LinkedNode::try_oneof(
            &mut inner,
            &[NodeTarget::Declaration(&[
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
            LinkedNode::try_oneof(parser, &[NodeTarget::Statement(&[StatementId::Block])])?
                .ok_or_else(|| E::MissedFnBlock.link_between(&sig, &close_tk))?;
        Ok(Some(FunctionDeclaration {
            sig,
            name,
            open: open_tk,
            close: close_tk,
            args,
            block: Box::new(block),
            uuid: Uuid::new_v4(),
        }))
    }
}
