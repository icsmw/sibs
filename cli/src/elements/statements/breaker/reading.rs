use crate::{
    elements::{Breaker, ElementRef},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Breaker> for Breaker {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Breaker>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Breaker);
        if reader.move_to().word(&[words::BREAK]).is_some() {
            Ok(Some(Breaker {
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Breaker, Breaker> for Breaker {}
