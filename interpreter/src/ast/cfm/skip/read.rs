use lexer::{Kind, KindId};

use crate::*;

use super::SkipTaskArgument;
/// #[skip([task_args], func())]

impl ReadNode<Skip> for Skip {
    fn read(parser: &mut Parser) -> Result<Option<Skip>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        let Kind::Identifier(ident) = &token.kind else {
            return Ok(None);
        };
        if ident != "skip" {
            return Ok(None);
        }
        let Some(mut inner) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Err(E::NoSkipDirectiveArgs);
        };
        let Some(mut args_inner) = inner.between(KindId::LeftBracket, KindId::RightBracket)? else {
            return Err(E::NoSkipDirectiveTaskArgs);
        };
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
            let Some(node) = Node::try_oneof(
                &mut args_inner,
                &[NodeReadTarget::Value(&[
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
            return Err(E::UnrecognizedCode(args_inner.to_string()));
        }
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma);
        }
        let _ = inner.token();
        let Some(func) =
            Expression::try_read(&mut inner, ExpressionId::FunctionCall)?.map(Node::Expression)
        else {
            return Err(E::NoSkipDirectiveFuncCall);
        };
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()));
        }
        Ok(Some(Skip {
            token,
            args,
            func: Box::new(func),
        }))
    }
}
