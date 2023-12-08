use crate::parser::{
    chars,
    entry::{Reader, Reading},
    E,
};
#[derive(Debug)]
pub struct Group {
    pub inner: String,
}

impl Reading<Group> for Group {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if reader.move_to_char(chars::OPEN_SQ_BRACKET)? {
            if let Some((inner, _, uuid)) =
                reader.read_until(&[chars::CLOSE_SQ_BRACKET], true, false)?
            {
                Ok(Some(Group::new(inner)?))
            } else {
                Err(E::NotClosedGroup)
            }
        } else {
            Ok(None)
        }
    }
}

impl Group {
    pub fn new(inner: String) -> Result<Self, E> {
        if inner.trim().is_empty() {
            Err(E::EmptyGroup)
        } else {
            Ok(Group { inner })
        }
    }
}
