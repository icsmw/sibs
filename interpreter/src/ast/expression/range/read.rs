use lexer::Kind;

use crate::*;

impl ReadNode<Range> for Range {
    fn read(parser: &mut Parser) -> Result<Option<Range>, LinkedErr<E>> {
        let Some(left) = Node::try_oneof(
            parser,
            &[
                NodeReadTarget::Value(&[ValueId::Number]),
                NodeReadTarget::Expression(&[ExpressionId::Variable]),
            ],
        )?
        else {
            return Ok(None);
        };
        if let Some(tk) = parser.token() {
            if !matches!(tk.kind, Kind::DotDot) {
                return Ok(None);
            }
        }
        let Some(right) = Node::try_oneof(
            parser,
            &[
                NodeReadTarget::Value(&[ValueId::Number]),
                NodeReadTarget::Expression(&[ExpressionId::Variable]),
            ],
        )?
        else {
            return Ok(None);
        };
        Ok(Some(Range {
            left: Box::new(left),
            right: Box::new(right),
        }))
    }
}
