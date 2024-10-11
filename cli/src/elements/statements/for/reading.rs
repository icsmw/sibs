use crate::{
    elements::{Element, ElementRef, For},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};
use tokio_util::sync::CancellationToken;

impl TryDissect<For> for For {
    fn try_dissect(reader: &mut Reader) -> Result<Option<For>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::For);
        if reader.move_to().word(&[words::FOR]).is_some() {
            let Some(index) = Element::include(reader, &[ElementRef::VariableName])? else {
                return Err(E::NoIndexInForLoop.by_reader(reader));
            };
            if reader.move_to().word(&[words::IN]).is_none() {
                return Err(E::NoINKeywordInForLoop.by_reader(reader));
            }
            let Some(target) = Element::include(
                reader,
                &[
                    ElementRef::Range,
                    ElementRef::VariableName,
                    ElementRef::Values,
                ],
            )?
            else {
                return Err(E::NoRangeInForLoop.by_reader(reader));
            };
            let Some(mut block) = Element::include(reader, &[ElementRef::Block])? else {
                return Err(E::NoBodyInForLoop.by_reader(reader));
            };
            if let Element::Block(block, _) = &mut block {
                block.set_owner(ElementRef::For);
                block.set_breaker(CancellationToken::new());
            }
            Ok(Some(Self {
                index: Box::new(index),
                target: Box::new(target),
                block: Box::new(block),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<For, For> for For {}
