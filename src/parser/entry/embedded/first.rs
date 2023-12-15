use crate::parser::{
    entry::{Block, Group, Reading},
    words, Reader, E,
};

#[derive(Debug)]
pub struct First {
    pub block: Block,
}

impl Reading<First> for First {
    fn read(reader: &mut Reader) -> Result<Option<First>, E> {
        if reader.move_to_word(&[words::FIRST])?.is_some() {
            if let Some(group) = Group::read(reader)? {
                Ok(Some(First {
                    block: Block::read(&mut reader.inherit(group.inner))?.ok_or(E::EmptyGroup)?,
                }))
            } else {
                Err(E::NoGroup)
            }
        } else {
            Ok(None)
        }
    }
}
