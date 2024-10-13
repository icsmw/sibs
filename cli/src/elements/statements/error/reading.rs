use crate::{
    elements::{Element, ElementId, Error},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Error> for Error {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Error>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Error);
        if reader.move_to().word(&[words::ERROR]).is_none() {
            return Ok(None);
        };
        let output = Element::include(reader, &[ElementId::PatternString])?
            .ok_or(E::NoErrorMessageDefinition.by_reader(reader))?;
        Ok(Some(Error {
            token: close(reader),
            output: Box::new(output),
        }))
    }
}

impl Dissect<Error, Error> for Error {}
