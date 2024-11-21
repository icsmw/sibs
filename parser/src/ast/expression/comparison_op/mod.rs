mod link;
#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use lexer::Kind;

impl ReadNode<ComparisonOp> for ComparisonOp {
    fn read(parser: &mut Parser) -> Result<Option<ComparisonOp>, LinkedErr<E>> {
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        let operator = match tk.kind {
            Kind::Less => ComparisonOperator::Less,
            Kind::LessEqual => ComparisonOperator::LessEqual,
            Kind::Greater => ComparisonOperator::Greater,
            Kind::GreaterEqual => ComparisonOperator::GreaterEqual,
            Kind::EqualEqual => ComparisonOperator::EqualEqual,
            Kind::BangEqual => ComparisonOperator::BangEqual,
            _ => {
                return Ok(None);
            }
        };
        Ok(Some(ComparisonOp {
            token: tk.clone(),
            operator,
        }))
    }
}
