use lexer::KindId;

use crate::*;

impl ReadNode<Block> for Block {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Block>, E> {
        let Some(inner) = parser.between(KindId::LeftBrace, KindId::RightBrace)? else {
            return Ok(None);
        };
        // Read nodes;
        Ok(None)
    }
}
