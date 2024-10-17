use crate::{
    elements::{Call, Element, ElementId},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Call> for Call {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Call>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Call);
        Ok(if reader.move_to().char(&[&chars::DOT]).is_some() {
            if let Some(chars::DOT) = reader.next().char() {
                None
            } else {
                let Some(el) = Element::read(reader, &[ElementId::Function])? else {
                    return Err(E::NoCallFunction.linked(&close(reader)));
                };
                Some(Call {
                    func: Box::new(el),
                    token: close(reader),
                })
            }
        } else {
            None
        })
    }
}

impl Dissect<Call, Call> for Call {}
