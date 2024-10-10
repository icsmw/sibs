use crate::{
    elements::SimpleString,
    error::LinkedErr,
    reader::{Dissect, Reader, TryDissect, E},
};

impl TryDissect<SimpleString> for SimpleString {
    fn try_dissect(reader: &mut Reader) -> Result<Option<SimpleString>, LinkedErr<E>> {
        reader.move_to().any();
        Ok(Some(SimpleString {
            value: reader.move_to().end(),
            token: reader.token()?.id,
        }))
    }
}

impl Dissect<SimpleString, SimpleString> for SimpleString {}
