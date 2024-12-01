#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind, KindId};

impl ReadNode<Task> for Task {
    fn read(parser: &mut Parser) -> Result<Option<Task>, LinkedErr<E>> {
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
        let (mut inner, ..) = parser
            .between(KindId::LeftParen, KindId::RightParen)?
            .ok_or_else(|| E::MissedTaskArguments.link_with_token(&sig))?;
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
                .ok_or_else(|| E::MissedTaskBlock.link_with_token(&sig))?;
        Ok(Some(Task {
            vis,
            sig,
            name,
            args,
            block: Box::new(block),
            uuid: Uuid::new_v4(),
        }))
    }
}
