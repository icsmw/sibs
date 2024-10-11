use crate::{
    elements::{Element, ElementRef, First},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<First> for First {
    fn try_dissect(reader: &mut Reader) -> Result<Option<First>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::First);
        if reader.move_to().word(&[words::FIRST]).is_some() {
            let Some(mut block) = Element::include(reader, &[ElementRef::Block])? else {
                return Err(E::NoFIRSTStatementBody.by_reader(reader));
            };
            if let Element::Block(block, _) = &mut block {
                block.set_owner(ElementRef::First);
            }
            Ok(Some(First {
                block: Box::new(block),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<First, First> for First {}
