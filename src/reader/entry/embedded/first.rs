use crate::reader::{
    chars,
    entry::{Block, Reading},
    words, Reader, E,
};

#[derive(Debug)]
pub struct First {
    pub block: Block,
    pub token: usize,
}

impl Reading<First> for First {
    fn read(reader: &mut Reader) -> Result<Option<First>, E> {
        if reader.move_to().word(&[&words::FIRST]).is_some() {
            if reader
                .group()
                .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                .is_some()
            {
                let mut token = reader.token()?;
                Ok(Some(First {
                    block: Block::read(&mut token.walker)?.ok_or(E::EmptyGroup)?,
                    token: token.id,
                }))
            } else {
                Err(E::NoGroup)
            }
        } else {
            Ok(None)
        }
    }
}
