#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Skip {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Identifier(..))
    }
}

impl ReadNode<Skip> for Skip {
    fn read(parser: &Parser) -> Result<Option<Skip>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        let Kind::Identifier(ident) = &token.kind else {
            return Ok(None);
        };
        if ident != "skip" {
            return Ok(None);
        }
        let (mut inner, open, close) = parser
            .between(KindId::LeftParen, KindId::RightParen)?
            .ok_or_else(|| E::NoSkipDirectiveArgs.link_with_token(&token))?;
        let mut args = Vec::new();
        let mut func = None;
        loop {
            if inner.is_next(KindId::Comma) {
                let _ = inner.token();
                continue;
            }
            let Some(node) = LinkedNode::try_oneof(
                &mut inner,
                &[
                    NodeTarget::Statement(&[StatementId::ArgumentAssignation]),
                    NodeTarget::Expression(&[ExpressionId::FunctionCall]),
                ],
            )?
            else {
                break;
            };
            if matches!(node.get_node(), Node::Statement(..)) {
                args.push(node);
            } else {
                func = Some(node);
                break;
            }
        }
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
        }
        let Some(func) = func else {
            return Err(E::NoSkipDirectiveFuncCall.link_until_end(&inner));
        };
        Ok(Some(Skip {
            token: token.clone(),
            args,
            func: Box::new(func),
            open: open.clone(),
            close: close.clone(),
            uuid: Uuid::new_v4(),
        }))
    }
}
