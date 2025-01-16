#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Skip {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Identifier(..))
    }
}

impl ReadNode<Skip> for Skip {
    fn read(parser: &mut Parser) -> Result<Option<Skip>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
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
        let (mut args_inner, ..) = inner
            .between(KindId::LeftBracket, KindId::RightBracket)?
            .ok_or_else(|| E::NoSkipDirectiveTaskArgs.link_until_end(&inner))?;
        let mut args = Vec::new();
        loop {
            if args_inner.is_next(KindId::Comma) {
                let _ = args_inner.token();
                continue;
            }
            if args_inner.is_next(KindId::Star) {
                args.push(SkipTaskArgument::Any);
                let _ = args_inner.token();
                continue;
            }
            let Some(node) = LinkedNode::try_oneof(
                &mut args_inner,
                &[NodeTarget::Value(&[
                    ValueId::Array,
                    ValueId::Boolean,
                    ValueId::Number,
                    ValueId::PrimitiveString,
                ])],
            )?
            else {
                break;
            };
            args.push(SkipTaskArgument::Value(node));
        }
        if !args_inner.is_done() {
            return Err(E::UnrecognizedCode(args_inner.to_string()).link_until_end(&args_inner));
        }
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma.link_by_current(&inner));
        }
        let _ = inner.token();
        let func = LinkedNode::try_oneof(
            &mut inner,
            &[NodeTarget::Expression(&[ExpressionId::FunctionCall])],
        )?
        .ok_or_else(|| E::NoSkipDirectiveFuncCall.link_until_end(&inner))?;
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
        }
        Ok(Some(Skip {
            token,
            args,
            func: Box::new(func),
            open,
            close,
            uuid: Uuid::new_v4(),
        }))
    }
}
