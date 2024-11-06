use lexer::Kind;

use crate::*;

impl ReadElement<Range> for Range {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Range>, E> {
        let Some(left) = Node::try_oneof(
            parser,
            nodes,
            &[
                NodeReadTarget::Value(&[ValueId::Number]),
                NodeReadTarget::Expression(&[ExpressionId::Variable]),
            ],
        )?
        else {
            return Ok(None);
        };
        parser.advance();
        if let Some(tk) = parser.token() {
            if !matches!(tk.kind, Kind::DotDot) {
                return Ok(None);
            }
        }
        parser.advance();
        let Some(right) = Node::try_oneof(
            parser,
            nodes,
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
