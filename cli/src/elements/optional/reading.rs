use crate::{
    elements::{Element, ElementId, Optional},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Optional> for Optional {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        if reader.rest().trim().starts_with(words::DO_ON) {
            return Ok(None);
        }
        let close = reader.open_token(ElementId::Optional);
        let condition = if let Some(el) = Element::include(
            reader,
            &[
                ElementId::Function,
                ElementId::VariableName,
                ElementId::Block,
                ElementId::Reference,
                ElementId::Comparing,
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
                ElementId::Function,
                ElementId::Reference,
                ElementId::VariableAssignation,
                ElementId::VariableName,
                ElementId::Block,
                ElementId::Each,
                ElementId::First,
                ElementId::PatternString,
                ElementId::Command,
                ElementId::Integer,
                ElementId::Boolean,
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
