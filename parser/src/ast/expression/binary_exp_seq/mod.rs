#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for BinaryExpSeq {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::Identifier(..) | Kind::Number(..) | Kind::LeftParen | Kind::Bang
        )
    }
}

impl ReadNode<BinaryExpSeq> for BinaryExpSeq {
    fn read(parser: &mut Parser) -> Result<Option<BinaryExpSeq>, LinkedErr<E>> {
        let mut collected: Vec<LinkedNode> = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            parser,
            &[
                NodeReadTarget::Value(&[ValueId::Number]),
                NodeReadTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::BinaryOp,
                    ExpressionId::BinaryExpGroup,
                ]),
            ],
        )? {
            if let Node::Expression(Expression::BinaryOp(op)) = &node.node {
                match collected.last().map(|n| &n.node) {
                    Some(Node::Expression(Expression::BinaryOp(prev))) => {
                        return Err(E::UnexpectedBinaryOperator(prev.token.id())
                            .link_with_token(&prev.token));
                    }
                    None => {
                        return Err(
                            E::UnexpectedBinaryOperator(op.token.id()).link_with_token(&op.token)
                        );
                    }
                    Some(_) => {}
                }
            } else {
                match collected.last().map(|n| &n.node) {
                    Some(Node::Expression(Expression::BinaryOp(..))) | None => {}
                    Some(Node::Expression(Expression::Variable(..))) => {
                        if collected.len() == 1 {
                            return Ok(None);
                        } else {
                            return Err(E::MissedBinaryOperator.link(&node));
                        }
                    }
                    Some(_) => {
                        return Err(E::MissedBinaryOperator.link(&node));
                    }
                }
            }
            collected.push(node);
        }
        if let Some(node) = collected.last() {
            if matches!(node.node, Node::Expression(Expression::BinaryOp(..))) {
                return Err(E::MissedBinaryArgument.link(node));
            }
        }
        let mut index = None;
        let mut finish = false;
        while !finish {
            for (n, node) in collected.iter().enumerate() {
                if let Node::Expression(Expression::BinaryOp(op)) = &node.node {
                    if matches!(op.operator, BinaryOperator::Slash | BinaryOperator::Star) {
                        index = Some(n);
                        break;
                    }
                }
            }
            if let Some(n) = index.take() {
                let right = collected.remove(n + 1);
                let operator = collected.remove(n);
                let left = collected.remove(n - 1);
                let from = left.md.link.pos.from;
                let to = right.md.link.pos.to;
                let src = left.md.link.src;
                let node = BinaryExp {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: Box::new(operator),
                    uuid: Uuid::new_v4(),
                };
                let mut expr = LinkedNode::from_node(Node::Expression(Expression::BinaryExp(node)));
                expr.md.link.pos.from = from;
                expr.md.link.pos.to = to;
                expr.md.link.src = src;
                collected.insert(n - 1, expr);
            } else {
                finish = true;
            }
        }
        Ok(if collected.is_empty() {
            None
        } else {
            Some(BinaryExpSeq {
                nodes: collected,
                uuid: Uuid::new_v4(),
            })
        })
    }
}
