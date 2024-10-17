use crate::{
    elements::{Element, ElementId, Reference},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Reference> for Reference {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Reference);
        if reader.move_to().char(&[&chars::COLON]).is_none() {
            return Ok(None);
        }
        let mut path: Vec<String> = Vec::new();
        let mut inputs: Vec<Element> = Vec::new();
        reader.trim();
        while let Some((content, stopped)) = reader.until().char(&[
            &chars::COLON,
            &chars::WS,
            &chars::OPEN_BRACKET,
            &chars::SEMICOLON,
            &chars::COMMA,
            &chars::DOT,
        ]) {
            if content.trim().is_empty() {
                Err(E::EmptyPathToReference.by_reader(reader))?
            }
            path.push(content);
            if stopped != chars::COLON {
                break;
            } else {
                reader.move_to().next();
                reader.trim();
            }
        }
        if !reader.rest().trim().is_empty()
            && Reader::is_ascii_alphabetic_and_alphanumeric(
                reader.rest().trim(),
                &[&chars::UNDERSCORE, &chars::DASH],
            )
        {
            path.push(reader.move_to().end());
        }
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            let inputs_token_id = reader.token()?.id;
            while let Some(el) = Element::read(
                &mut inner,
                &[
                    ElementId::VariableName,
                    ElementId::Integer,
                    ElementId::Boolean,
                    ElementId::PatternString,
                ],
            )? {
                inputs.push(el);
                let _ = inner.move_to().char(&[&chars::COMMA]);
            }
            if !inner.is_empty() {
                Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner))?;
            }
            if inputs.is_empty() {
                return Err(E::InvalidArgumentForReference.linked(&inputs_token_id));
            }
        }
        let token = close(reader);
        for part in path.iter() {
            if !Reader::is_ascii_alphabetic_and_alphanumeric(
                part,
                &[&chars::UNDERSCORE, &chars::DASH],
            ) {
                Err(E::InvalidReference(part.to_owned()).linked(&token))?
            }
        }
        Ok(Some(Reference {
            token,
            path,
            inputs,
        }))
    }
}

impl Dissect<Reference, Reference> for Reference {}
