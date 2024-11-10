use lexer::KindId;

use crate::*;

impl ReadNode<Block> for Block {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Block>, E> {
        let Some(_inner) = parser.between(KindId::LeftBrace, KindId::RightBrace)? else {
            return Ok(None);
        };
        // Read nodes;
        Ok(Some(Block { nodes: Vec::new() }))
    }
}
