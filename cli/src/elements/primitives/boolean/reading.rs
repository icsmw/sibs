use crate::{
    elements::Boolean,
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Boolean> for Boolean {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Boolean>, LinkedErr<E>> {
        reader.move_to().any();
        if let Some(value) = reader.move_to().word(&[words::TRUE, words::FALSE]) {
            Ok(Some(Boolean {
                value: value == words::TRUE,
                token: reader.token()?.id,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Boolean, Boolean> for Boolean {}
