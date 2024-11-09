use crate::*;
use lexer::KindId;

impl ReadNode<BinaryExpGroup> for BinaryExpGroup {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<BinaryExpGroup>, E> {
        let Some(mut inner) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
        };
        let mut collected = Vec::new();
        while let Some(node) = Expression::try_oneof(
            &mut inner,
            &[
                ExpressionId::BinaryExpPri,
                ExpressionId::BinaryOp,
                ExpressionId::BinaryExpGroup,
            ],
            nodes,
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
        Ok(if collected.is_empty() {
            None
        } else {
            Some(BinaryExpGroup { nodes: collected })
        })
    }
}
