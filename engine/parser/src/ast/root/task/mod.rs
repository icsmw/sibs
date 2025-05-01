#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Task {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::Keyword(Keyword::Task) | Kind::Keyword(Keyword::Private) | Kind::Pound
        )
    }
}

impl ReadNode<Task> for Task {
    fn read(parser: &Parser) -> Result<Option<Task>, LinkedErr<E>> {
        let mut gts = Vec::new();
        loop {
            'semicolons: loop {
                if parser.is_next(KindId::Semicolon) {
                    let _ = parser.token();
                } else {
                    break 'semicolons;
                }
            }
            let Some(node) = LinkedNode::try_oneof(
                parser,
                &[NodeTarget::ControlFlowModifier(&[
                    ControlFlowModifierId::Gatekeeper,
                ])],
            )?
            else {
                break;
            };
            gts.push(node);
        }
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        let (sig, vis) = if matches!(token.kind, Kind::Keyword(Keyword::Private)) {
            let sig = parser
                .token()
                .cloned()
                .ok_or_else(|| E::InvalidPrivateKeyUsage.link_with_token(&token))?;
            (sig, Some(token))
        } else {
            (token, None)
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Task)) {
            return Ok(None);
        }
        let name = parser
            .token()
            .cloned()
            .ok_or_else(|| E::MissedTaskName.link_with_token(&sig))?;
        if !matches!(name.kind, Kind::Identifier(..)) {
            return Err(E::MissedTaskName.link_with_token(&sig));
        }
        let (mut inner, open, close) = parser
            .between(KindId::LeftParen, KindId::RightParen)?
            .ok_or_else(|| E::MissedTaskArguments.link_with_token(&sig))?;
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
        let block = LinkedNode::try_oneof(parser, &[NodeTarget::Statement(&[StatementId::Block])])?
            .ok_or_else(|| E::MissedTaskBlock.link_with_token(&sig))?;
        Ok(Some(Task {
            vis,
            sig,
            name,
            open,
            close,
            args,
            gts,
            block: Box::new(block),
            uuid: Uuid::new_v4(),
        }))
    }
}
