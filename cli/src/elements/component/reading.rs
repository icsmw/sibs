use crate::{
    elements::{Component, Element, ElementId, SimpleString},
    error::LinkedErr,
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};
use std::path::PathBuf;
use uuid::Uuid;

impl TryDissect<Component> for Component {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Component>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Component);
        let Some((before, _)) = reader.until().char(&[&chars::POUND_SIGN]) else {
            return Ok(None);
        };
        if !before.is_empty() {
            Err(E::UnrecognizedCode(before).by_reader(reader))?;
        }
        let _ = reader.move_to().char(&[&chars::POUND_SIGN]);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_none()
        {
            return Err(E::NoComponentDefinition.by_reader(reader));
        }
        let mut inner = reader.token()?.bound;
        let name = inner
            .until()
            .char(&[&chars::COLON])
            .map(|(v, _)| {
                inner.move_to().next();
                v
            })
            .unwrap_or_else(|| inner.move_to().end());
        if name.trim().is_empty() {
            Err(E::EmptyComponentName.by_reader(reader))?;
        }
        if !Reader::is_ascii_alphabetic_and_alphanumeric(&name, &[&chars::UNDERSCORE, &chars::DASH])
        {
            Err(E::InvalidComponentName(name.clone()).by_reader(reader))?;
        }
        let (name, name_token) = (name, inner.token()?.id);
        let path = inner.rest().trim().to_string();
        let rest = if let Some((rest, _)) = reader.until().word(&[words::COMP]) {
            rest
        } else {
            reader.move_to().end()
        };
        if rest.trim().is_empty() {
            Err(E::NoComponentBody.linked(&name_token))?
        }
        let mut inner = reader.token()?.bound;
        let inner_token_id = reader.token()?.id;
        let mut elements: Vec<Element> = Vec::new();
        while let Some(el) =
            Element::include(&mut inner, &[ElementId::Task, ElementId::Gatekeeper])?
        {
            let _ = inner.move_to().char(&[&chars::SEMICOLON]);
            elements.push(el);
        }
        if elements.is_empty() {
            return Err(E::UnrecognizedCode(rest).linked(&inner_token_id));
        }
        Ok(Some(Component {
            uuid: Uuid::new_v4(),
            name: SimpleString {
                value: name,
                token: name_token,
            },
            elements,
            cwd: if path.is_empty() {
                None
            } else {
                Some(PathBuf::from(path))
            },
            token: close(reader),
        }))
    }
}

impl Dissect<Component, Component> for Component {}
