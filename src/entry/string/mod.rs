pub mod command;
pub mod pattern;
pub mod simple;

pub use command::*;
pub use pattern::*;
pub use simple::*;

use crate::{
    entry::{ElTarget, Element},
    error::LinkedErr,
    reader::{self, chars, Reader, E},
};

pub type PatternStringResult = (String, Vec<(String, Element)>, usize);

pub fn read(
    reader: &mut Reader,
    wrapper: char,
) -> Result<Option<PatternStringResult>, LinkedErr<E>> {
    let restore = reader.pin();
    reader.trim();
    let close = reader.open_token();
    if reader.move_to().char(&[&wrapper]).is_some() {
        let mut injections: Vec<(String, Element)> = vec![];
        let mut closed = false;
        while let Some((_, stopped)) = reader.until().char(&[&chars::TYPE_OPEN, &wrapper]) {
            if stopped == wrapper {
                closed = true;
                break;
            } else if let Some(hook) = reader
                .group()
                .between(&chars::TYPE_OPEN, &chars::TYPE_CLOSE)
            {
                let mut inner = reader.token()?.bound;
                if let Some(el) = Element::include(
                    &mut inner,
                    &[ElTarget::VariableName, ElTarget::Function, ElTarget::If],
                )? {
                    injections.push((hook, el));
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
            Ok(Some((content, injections, token)))
        }
    } else {
        restore(reader);
        Ok(None)
    }
}
