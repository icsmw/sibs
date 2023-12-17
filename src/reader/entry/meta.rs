use crate::reader::{
    chars,
    entry::{Reader, Reading},
    words, E,
};
#[derive(Debug)]
pub struct Meta {
    pub inner: Vec<String>,
    pub index: usize,
}

impl Reading<Meta> for Meta {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        let mut inner: Vec<String> = vec![];
        let from = reader.pos;
        while reader.move_to_word(&[words::META])?.is_some() {
            if let Some((line, _, _)) = reader.read_until(&[chars::CARET], true, true)? {
                inner.push(line.trim().to_string());
            } else {
                Err(E::NoMetaContent)?
            }
        }
        if inner.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Meta {
                inner,
                index: reader.get_index_until_current(from),
            }))
        }
    }
}
