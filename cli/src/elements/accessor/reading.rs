use crate::{
    elements::{Accessor, Element, ElementId},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Accessor> for Accessor {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Accessor>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Accessor);
        Ok(
            if reader
                .group()
                .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                let Some(el) = Element::read(
                    &mut inner,
                    &[
                        ElementId::Integer,
                        ElementId::Function,
                        ElementId::VariableName,
                    ],
                )?
                else {
                    return Err(E::NoElementAccessor.linked(&close(reader)));
                };
                Some(Accessor {
                    index: Box::new(el),
                    token: close(reader),
                })
            } else {
                None
            },
        )
    }
}

impl Dissect<Accessor, Accessor> for Accessor {}
