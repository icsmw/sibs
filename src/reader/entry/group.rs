use crate::reader::{
    chars,
    entry::{Reader, Reading},
    E,
};
#[derive(Debug)]
pub struct Group {
    pub inner: String,
    pub index: usize,
}

impl Reading<Group> for Group {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if let Some((inner, index)) =
            reader.read_until_close(chars::OPEN_SQ_BRACKET, chars::CLOSE_SQ_BRACKET, true)?
        {
            Ok(Some(if inner.trim().is_empty() {
                Err(E::EmptyGroup)?
            } else {
                Group { inner, index }
            }))
        } else {
            Ok(None)
        }
    }
}
