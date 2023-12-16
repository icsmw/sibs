use crate::reader::{
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
        if let Some((inner, uuid)) =
            reader.read_until_close(chars::OPEN_SQ_BRACKET, chars::CLOSE_SQ_BRACKET, true)?
        {
            Ok(Some(Group::new(inner)?))
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
