use crate::{
    elements::{Element, ElementRef, SimpleString, Task},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Task> for Task {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Task);
        let Some(_) = reader.move_to().char(&[&chars::AT]) else {
            return Ok(None);
        };
        let Some((name, stopped_on)) = reader
            .until()
            .char(&[&chars::OPEN_BRACKET, &chars::OPEN_CURLY_BRACE])
        else {
            return Ok(None);
        };
        let (name, name_token) = (name.trim().to_string(), reader.token()?.id);
        if stopped_on == chars::OPEN_BRACKET {
            reader.move_to().next();
        }
        if !Reader::is_ascii_alphabetic_and_alphanumeric(&name, &[&chars::UNDERSCORE, &chars::DASH])
        {
            Err(E::InvalidTaskName(name.clone()).linked(&name_token))?
        }
        let declarations: Vec<Element> = if stopped_on == chars::OPEN_CURLY_BRACE {
            Vec::new()
        } else if reader.until().char(&[&chars::CLOSE_BRACKET]).is_some() {
            reader.move_to().next();
            let mut declarations: Vec<Element> = Vec::new();
            let mut inner = reader.token()?.bound;
            while let Some(el) = Element::include(&mut inner, &[ElementRef::VariableDeclaration])? {
                let _ = inner.move_to().char(&[&chars::COMMA]);
                declarations.push(el);
            }
            if !inner.is_empty() {
                Err(E::InvalidTaskArguments(inner.rest().trim().to_string()).by_reader(&inner))?
            }
            declarations
        } else {
            Err(E::NoTaskArguments.linked(&name_token))?
        };
        let mut dependencies: Vec<Element> = Vec::new();
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            while let Some(el) = Element::include(
                &mut inner,
                &[ElementRef::Reference, ElementRef::VariableAssignation],
            )? {
                let _ = inner.move_to().char(&[&chars::SEMICOLON]);
                dependencies.push(el);
            }
            if !inner.is_empty() {
                Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner))?;
            }
        }
        if let Some(block) = Element::include(reader, &[ElementRef::Block])? {
            Ok(Some(Task {
                name: SimpleString {
                    value: name,
                    token: name_token,
                },
                declarations,
                dependencies,
                token: close(reader),
                block: Box::new(block),
            }))
        } else {
            Err(E::FailFindTaskActions.linked(&name_token))
        }
    }
}

impl Dissect<Task, Task> for Task {}
