pub mod command;
pub mod pattern;
pub mod simple;

pub use command::*;
pub use pattern::*;
pub use simple::*;

use crate::{
    elements::{Element, ElementId},
    error::LinkedErr,
    reader::{chars, Reader, E},
};

pub type PatternStringResult = (String, Vec<Element>, usize);

pub fn read(
    reader: &mut Reader,
    wrapper: char,
    el: ElementId,
) -> Result<Option<PatternStringResult>, LinkedErr<E>> {
    reader.trim();
    let close = reader.open_token(el);
    if reader.move_to().char(&[&wrapper]).is_none() {
        return Ok(None);
    }
    let mut elements: Vec<Element> = Vec::new();
    let mut closed = false;
    while let Some((_, stopped)) = reader.until().char(&[&chars::OPEN_CURLY_BRACE, &wrapper]) {
        let inner_token = reader.token()?;
        if stopped == wrapper {
            elements.push(SimpleString::as_element(
                inner_token.content,
                &inner_token.id,
            ));
            closed = true;
            break;
        } else if reader
            .group()
            .between(&chars::OPEN_CURLY_BRACE, &chars::CLOSE_CURLY_BRACE)
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            if let Some(el) = Element::include(
                &mut inner,
                &[
                    ElementId::VariableName,
                    ElementId::Function,
                    ElementId::If,
                ],
            )? {
                elements.extend_from_slice(&[
                    SimpleString::as_element(inner_token.content, &inner_token.id),
                    el,
                ]);
                if !inner.is_empty() {
                    Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner))?
                }
            } else {
                Err(E::FailToFindInjection.by_reader(&inner))?
            }
        } else {
            Err(E::NoInjectionClose.by_reader(reader))?
        }
    }
    let _ = reader.move_to().next();
    let token = close(reader);
    if !closed {
        Err(E::NoStringEnd.linked(&token))?
    } else {
        let mut content = reader.get_fragment(&token)?.content;
        if content.starts_with(wrapper) {
            let _ = content.remove(0);
        }
        if content.ends_with(wrapper) {
            let _ = content.remove(content.len() - 1);
        }
        Ok(Some((content, elements, token)))
    }
}
