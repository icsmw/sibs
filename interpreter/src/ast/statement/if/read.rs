use lexer::KindId;

use crate::*;

impl ReadNode<If> for If {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<If>, E> {
        let mut cases = Vec::new();
        loop {
            let restore = parser.pin();
            if let Some(tk) = parser.token().cloned() {
                match tk.id() {
                    KindId::If => {
                        let Some(cond) =
                            Expression::try_read(parser, ExpressionId::ComparisonSeq, nodes)?
                                .map(Node::Expression)
                        else {
                            return Err(E::MissedExpectation(
                                tk.id().to_string(),
                                ExpressionId::ComparisonSeq.to_string(),
                            ));
                        };
                        let Some(blk) = Statement::try_read(parser, StatementId::Block, nodes)?
                            .map(Node::Statement)
                        else {
                            return Err(E::MissedExpectation(
                                tk.id().to_string(),
                                StatementId::Block.to_string(),
                            ));
                        };
                        cases.push(IfCase::If(cond, blk, tk));
                    }
                    KindId::Else => {
                        let Some(blk) = Statement::try_read(parser, StatementId::Block, nodes)?
                            .map(Node::Statement)
                        else {
                            return Err(E::MissedExpectation(
                                tk.id().to_string(),
                                StatementId::Block.to_string(),
                            ));
                        };
                        cases.push(IfCase::Else(blk, tk));
                    }
                    _ => {
                        restore(parser);
                        break;
                    }
                }
            } else {
                break;
            }
        }
        if cases.is_empty() {
            Ok(None)
        } else {
            Ok(Some(If { cases }))
        }
    }
}
