use crate::{
    elements::{function::Function, Element, ElementId},
    error::LinkedErr,
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Function> for Function {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        reader.move_to().any();
        let close = reader.open_token(ElementId::Function);
        let Some((name, mut stop)) = reader.until().char(&[
            &chars::OPEN_BRACKET,
            &chars::CARET,
            &chars::SEMICOLON,
            &chars::COMMA,
            &chars::WS,
            &chars::OPEN_BRACKET,
            &chars::CLOSE_BRACKET,
        ]) else {
            return Ok(None);
        };
        if stop == chars::WS {
            if reader.move_to().char(&[&chars::OPEN_BRACKET]).is_none() {
                return Ok(None);
            } else {
                let _ = reader.move_to().prev();
                stop = chars::OPEN_BRACKET;
            }
        }
        if stop != chars::OPEN_BRACKET
            || !Reader::is_ascii_alphabetic_and_alphanumeric(
                &name,
                &[&chars::UNDERSCORE, &chars::DASH, &chars::COLON],
            )
            || name.trim().chars().any(|c| c.is_whitespace())
            || name
                .trim()
                .chars()
                .next()
                .map(|c| !c.is_ascii_alphabetic())
                .unwrap_or(true)
            || words::is_reserved(name.trim())
            || name.is_empty()
            || name
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
        {
            return Ok(None);
        }
        let args_close = reader.open_token(ElementId::Function);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_none()
        {
            return Err(E::NotClosedFunctionArgs.by_reader(reader));
        }
        let mut elements: Vec<Element> = Vec::new();
        let mut inner = reader.token()?.bound;
        while !inner.is_empty() {
            if let Some(el) = Element::read(
                &mut inner,
                &[
                    ElementId::Closure,
                    ElementId::Values,
                    ElementId::Function,
                    ElementId::If,
                    ElementId::PatternString,
                    ElementId::Reference,
                    ElementId::Comparing,
                    ElementId::VariableName,
                    ElementId::Command,
                    ElementId::Integer,
                    ElementId::Boolean,
                ],
            )? {
                if inner.move_to().char(&[&chars::COMMA]).is_none() && !inner.is_empty() {
                    Err(E::MissedComma.by_reader(&inner))?;
                }
                elements.push(el);
            } else if let Some((content, _)) = inner.until().char(&[&chars::COMMA]) {
                if content.trim().is_empty() {
                    Err(E::NoContentBeforeComma.by_reader(&inner))?;
                }
                elements.push(
                    Element::read(&mut inner.token()?.bound, &[ElementId::SimpleString])?
                        .ok_or(E::NoContentBeforeComma.by_reader(&inner))?,
                );
                let _ = inner.move_to().char(&[&chars::COMMA]);
            } else if !inner.is_empty() {
                elements.push(
                    Element::read(&mut inner, &[ElementId::SimpleString])?
                        .ok_or(E::NoContentBeforeComma.by_reader(&inner))?,
                );
            }
        }
        if reader.rest().trim().starts_with(words::DO_ON) {
            return Ok(Some(Self::new(
                close(reader),
                args_close(reader),
                elements,
                name,
            )?));
        }
        Ok(Some(Self::new(
            close(reader),
            args_close(reader),
            elements,
            name,
        )?))
    }
}

impl Dissect<Function, Function> for Function {}
