use crate::*;

impl ReadNode<BinaryExpSeq> for BinaryExpSeq {
    fn read(parser: &mut Parser) -> Result<Option<BinaryExpSeq>, E> {
        let mut collected = Vec::new();
        while let Some(node) = Expression::try_oneof(
            parser,
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
        Ok(if collected.is_empty() {
            None
        } else {
            Some(BinaryExpSeq { nodes: collected })
        })
    }
}
