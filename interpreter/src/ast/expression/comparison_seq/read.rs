use crate::*;

impl ReadNode<ComparisonSeq> for ComparisonSeq {
    fn read(parser: &mut Parser) -> Result<Option<ComparisonSeq>, E> {
        let mut collected = Vec::new();
        while let Some(node) = Expression::try_oneof(
            parser,
            &[
                ExpressionId::Comparison,
                ExpressionId::LogicalOp,
                ExpressionId::ComparisonGroup,
            ],
        )?
        .map(Node::Expression)
        {
            if let Node::Expression(Expression::LogicalOp(op)) = &node {
                match collected.last() {
                    Some(Node::Expression(Expression::LogicalOp(prev))) => {
                        return Err(E::UnexpectedLogicalOperator(prev.token.id()));
                    }
                    None => {
                        return Err(E::UnexpectedLogicalOperator(op.token.id()));
                    }
                    Some(_) => {}
                }
            } else {
                match collected.last() {
                    Some(Node::Expression(Expression::LogicalOp(..))) | None => {}
                    Some(_) => {
                        return Err(E::MissedLogicalOperator);
                    }
                }
            }
            collected.push(node);
        }
        Ok(if collected.is_empty() {
            None
        } else {
            Some(ComparisonSeq { nodes: collected })
        })
    }
}
