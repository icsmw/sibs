use crate::{
    elements::Meta,
    error::LinkedErr,
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Meta> for Meta {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let mut inner: Vec<String> = Vec::new();
        while reader.move_to().expression(&[words::META]).is_some() {
            if let Some((line, _)) = reader.until().char(&[&chars::CARET]) {
                inner.push(line.trim().to_string());
            } else {
                Err(E::NoMetaContent.by_reader(reader))?
            }
        }
        if inner.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Meta {
                inner,
                token: reader.token()?.id,
            }))
        }
    }
}

impl Dissect<Meta, Meta> for Meta {}
