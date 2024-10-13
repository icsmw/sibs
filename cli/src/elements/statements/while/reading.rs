use crate::{
    elements::{Element, ElementId, While},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};
use tokio_util::sync::CancellationToken;

impl TryDissect<While> for While {
    fn try_dissect(reader: &mut Reader) -> Result<Option<While>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::While);
        if reader.move_to().word(&[words::WHILE]).is_some() {
            let Some(condition) = Element::include(reader, &[ElementId::Comparing])? else {
                return Err(E::NoConditionInWhile.by_reader(reader));
            };
            let Some(mut block) = Element::include(reader, &[ElementId::Block])? else {
                return Err(E::NoBodyInForLoop.by_reader(reader));
            };
            if let Element::Block(block, _) = &mut block {
                block.set_owner(ElementId::While);
                block.set_breaker(CancellationToken::new());
            }
            Ok(Some(Self {
                condition: Box::new(condition),
                block: Box::new(block),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<While, While> for While {}
