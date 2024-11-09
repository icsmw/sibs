use lexer::KindId;

use crate::*;

impl ReadNode<Accessor> for Accessor {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Accessor>, E> {
        let Some(mut inner) = parser.between(KindId::LeftBracket, KindId::RightBracket)? else {
            return Ok(None);
        };
        let Some(node) = Node::try_oneof(
            &mut inner,
            nodes,
            &[
                NodeReadTarget::Value(&[ValueId::Number]),
                NodeReadTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::BinaryExpSeq,
                    ExpressionId::FunctionCall,
                ]),
                NodeReadTarget::Statement(&[StatementId::If]),
            ],
        )?
        else {
            return Ok(None);
        };
        if !parser.is_done() {
            return Ok(None);
        }
        Ok(Some(Accessor {
            node: Box::new(node),
        }))
    }
}
