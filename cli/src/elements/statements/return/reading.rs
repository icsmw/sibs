use crate::{
    elements::{Element, ElementId, Return},
    error::LinkedErr,
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Return> for Return {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Return>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Return);
        if reader.move_to().word(&[words::RETURN]).is_none() {
            return Ok(None);
        }
        let output = if let Some(output) = Element::read(
            reader,
            &[
                ElementId::Values,
                ElementId::VariableName,
                ElementId::Error,
                ElementId::Function,
                ElementId::If,
                ElementId::Integer,
                ElementId::Boolean,
                ElementId::PatternString,
            ],
        )? {
            Some(Box::new(output))
        } else {
            let pin = reader.pin();
            let semicolon = reader.move_to().char(&[&chars::SEMICOLON]).is_some();
            pin(reader);
            if !semicolon {
                return Err(E::MissedReturnOutputOrMissedSemicolon.by_reader(reader));
            } else {
                None
            }
        };
        Ok(Some(Return {
            token: close(reader),
            output,
        }))
    }
}

impl Dissect<Return, Return> for Return {}
