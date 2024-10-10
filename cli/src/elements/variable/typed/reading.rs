use crate::{
    elements::{ElementRef, VariableType},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<VariableType> for VariableType {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableType>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::VariableType);
        if reader.move_to().char(&[&chars::OPEN_CURLY_BRACE]).is_none() {
            return Ok(None);
        }
        if let Some((word, _char)) = reader.until().char(&[&chars::CLOSE_CURLY_BRACE]) {
            reader.move_to().next();
            Ok(Some(VariableType::new(word, close(reader))?))
        } else {
            Err(E::NotClosedTypeDeclaration.by_reader(reader))
        }
    }
}

impl Dissect<VariableType, VariableType> for VariableType {}
