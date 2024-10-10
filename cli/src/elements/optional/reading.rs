use crate::{
    elements::{Element, ElementRef, Optional},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Optional> for Optional {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        if reader.rest().trim().starts_with(words::DO_ON) {
            return Ok(None);
        }
        let close = reader.open_token(ElementRef::Optional);
        let condition = if let Some(el) = Element::include(
            reader,
            &[
                ElementRef::Function,
                ElementRef::VariableName,
                ElementRef::Block,
                ElementRef::Reference,
                ElementRef::Comparing,
            ],
        )? {
            Box::new(el)
        } else {
            return Ok(None);
        };
        if !reader.rest().trim().starts_with(words::DO_ON) {
            return Ok(None);
        }
        if reader.move_to().expression(&[words::DO_ON]).is_none() {
            return Err(E::NoOptionalRedirection.by_reader(reader));
        }
        let action = if let Some(el) = Element::include(
            reader,
            &[
                ElementRef::Function,
                ElementRef::Reference,
                ElementRef::VariableAssignation,
                ElementRef::VariableName,
                ElementRef::Block,
                ElementRef::Each,
                ElementRef::First,
                ElementRef::PatternString,
                ElementRef::Command,
                ElementRef::Integer,
                ElementRef::Boolean,
            ],
        )? {
            Box::new(el)
        } else {
            return Err(E::FailFindActionForOptional.by_reader(reader));
        };
        Ok(Some(Optional {
            token: close(reader),
            action,
            condition,
        }))
    }
}

impl Dissect<Optional, Optional> for Optional {}
