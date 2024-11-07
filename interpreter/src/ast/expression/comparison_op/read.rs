use lexer::Kind;

use crate::*;

impl ReadElement<ComparisonOp> for ComparisonOp {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<ComparisonOp>, E> {
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        let operator = match tk.kind {
            Kind::Less => ComparisonOperator::Less,
            Kind::LessEqual => ComparisonOperator::LessEqual,
            Kind::Greater => ComparisonOperator::Greater,
            Kind::GreaterEqual => ComparisonOperator::GreaterEqual,
            Kind::Equals => ComparisonOperator::Equal,
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
