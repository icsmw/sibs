use crate::*;
use lexer::KindId;

impl ReadNode<BinaryExpGroup> for BinaryExpGroup {
    fn read(parser: &mut Parser) -> Result<Option<BinaryExpGroup>, E> {
        let Some(mut inner) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
        };
        let mut collected = Vec::new();
        while let Some(node) = Expression::try_oneof(
            &mut inner,
            &[
                ExpressionId::BinaryExp,
                ExpressionId::BinaryOp,
                ExpressionId::BinaryExpGroup,
            ],
        )?
        .map(Node::Expression)
        {
            if let Node::Expression(Expression::BinaryOp(op)) = &node {
                match collected.last() {
                    Some(Node::Expression(Expression::BinaryOp(prev))) => {
                        return Err(E::UnexpectedBinaryOperator(prev.token.id()));
                    }
                    None => {
                        return Err(E::UnexpectedBinaryOperator(op.token.id()));
                    }
                    Some(_) => {}
                }
            } else {
                match collected.last() {
                    Some(Node::Expression(Expression::BinaryOp(..))) | None => {}
                    Some(_) => {
                        return Err(E::MissedBinaryOperator);
                    }
                }
            }
            collected.push(node);
        }
        Ok(if collected.is_empty() || !inner.is_done() {
            None
        } else {
            Some(BinaryExpGroup { nodes: collected })
        })
    }
}
