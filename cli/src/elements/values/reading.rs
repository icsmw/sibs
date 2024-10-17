use crate::{
    elements::{Element, ElementId, Values},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Values> for Values {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Values>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Values);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_none()
        {
            return Ok(None);
        }
        let token = reader.token()?;
        let mut inner = token.bound;
        let mut elements: Vec<Element> = Vec::new();
        if inner.rest().trim().is_empty() {
            return Ok(Some(Values {
                token: close(reader),
                elements,
            }));
        }
        while let Some(el) = Element::read(
            &mut inner,
            &[
                ElementId::Command,
                ElementId::Function,
                ElementId::PatternString,
                ElementId::Reference,
                ElementId::Values,
                ElementId::VariableName,
                ElementId::Integer,
                ElementId::Boolean,
            ],
        )? {
            if inner.move_to().char(&[&chars::COMMA]).is_none() && !inner.rest().trim().is_empty() {
                Err(E::MissedComma.by_reader(&inner))?;
            }
            elements.push(el);
        }
        if !inner.rest().trim().is_empty() {
            if let Some((content, _)) = inner.until().char(&[&chars::COMMA]) {
                Err(E::UnrecognizedCode(content).by_reader(&inner))?;
            } else {
                Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner))?;
            }
        }
        Ok(Some(Values {
            token: close(reader),
            elements,
        }))
    }
}

impl Dissect<Values, Values> for Values {}
