pub mod command;
pub mod pattern;
pub mod simple;

pub use command::*;
pub use pattern::*;
pub use simple::*;

use crate::{
    entry::{ElTarget, Element},
    error::LinkedErr,
    reader::{chars, Reader, E},
};

pub type PatternStringResult = (String, Vec<(String, Element)>, usize);

pub fn read(
    reader: &mut Reader,
    wrapper: char,
) -> Result<Option<PatternStringResult>, LinkedErr<E>> {
    let close = reader.open_token();
    if let Some(pattern) = reader.group().closed(&wrapper) {
        let mut injections: Vec<(String, Element)> = vec![];
        let mut inner = reader.token()?.bound;
        while inner.seek_to().char(&chars::TYPE_OPEN) {
            if let Some(hook) = inner.group().between(&chars::TYPE_OPEN, &chars::TYPE_CLOSE) {
                let mut inner = inner.token()?.bound;
                if let Some(el) = Element::include(
                    &mut inner,
                    &[ElTarget::VariableName, ElTarget::Function, ElTarget::If],
                )? {
                    injections.push((hook, el));
                } else {
                    Err(E::FailToFineInjection.by_reader(&inner))?
                }
            } else {
                Err(E::NoInjectionClose.by_reader(reader))?
            }
        }
        Ok(Some((pattern, injections, close(reader))))
    } else {
        Ok(None)
    }
}
