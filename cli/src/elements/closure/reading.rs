use crate::{
    elements::{Closure, Element, ElementRef},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use uuid::Uuid;

impl TryDissect<Closure> for Closure {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        reader.move_to().any();
        let close = reader.open_token(ElementRef::Closure);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_none()
        {
            return Ok(None);
        }
        let mut args_inner = reader.token()?.bound;
        let Some(block) = Element::include(reader, &[ElementRef::Block])? else {
            return Ok(None);
        };
        let mut args = Vec::new();
        while !args_inner.is_empty() {
            if let Some(el) = Element::include(&mut args_inner, &[ElementRef::VariableName])? {
                if args_inner.move_to().char(&[&chars::COMMA]).is_none() && !args_inner.is_empty() {
                    Err(E::MissedComma.by_reader(&args_inner))?;
                }
                args.push(el);
            } else {
                return Err(E::InvalidClosureArgument.by_reader(&args_inner));
            }
        }
        Ok(Some(Self {
            token: close(reader),
            block: Box::new(block),
            args,
            uuid: Uuid::new_v4(),
        }))
    }
}

impl Dissect<Closure, Closure> for Closure {}
