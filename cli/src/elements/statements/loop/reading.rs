use crate::{
    elements::{Element, ElementId, Loop},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};
use tokio_util::sync::CancellationToken;

impl TryDissect<Loop> for Loop {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Loop>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Loop);
        if reader.move_to().word(&[words::LOOP]).is_some() {
            let Some(mut block) = Element::read(reader, &[ElementId::Block])? else {
                return Err(E::NoBodyInForLoop.by_reader(reader));
            };
            if let Element::Block(block, _) = &mut block {
                block.set_owner(ElementId::Loop);
                block.set_breaker(CancellationToken::new());
            }
            Ok(Some(Self {
                block: Box::new(block),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Loop, Loop> for Loop {}
