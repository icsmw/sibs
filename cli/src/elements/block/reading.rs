use crate::{
    elements::{Block, Element, ElementId},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Block> for Block {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Block>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Block);
        if reader
            .group()
            .between(&chars::OPEN_CURLY_BRACE, &chars::CLOSE_CURLY_BRACE)
            .is_none()
        {
            return Ok(None);
        }
        let mut inner = reader.token()?.bound;
        let block_token_id = reader.token()?.id;
        let mut elements: Vec<Element> = Vec::new();
        loop {
            if let Some(el) = Element::read(
                &mut inner,
                &[
                    ElementId::Function,
                    ElementId::If,
                    ElementId::Each,
                    ElementId::Breaker,
                    ElementId::First,
                    ElementId::Join,
                    ElementId::VariableAssignation,
                    ElementId::Optional,
                    ElementId::Reference,
                    ElementId::PatternString,
                    ElementId::VariableName,
                    ElementId::Values,
                    ElementId::Command,
                    ElementId::Integer,
                    ElementId::Boolean,
                    ElementId::For,
                    ElementId::Return,
                    ElementId::Incrementer,
                    ElementId::Loop,
                    ElementId::While,
                    ElementId::Conclusion,
                    ElementId::Comment,
                ],
            )? {
                if inner.move_to().char(&[&chars::SEMICOLON]).is_none() {
                    return if let Some((content, _)) = inner.until().char(&[&chars::SEMICOLON]) {
                        Err(E::UnrecognizedCode(content).by_reader(&inner))
                    } else {
                        Err(E::MissedSemicolon.by_reader(&inner))
                    };
                }
                elements.push(el);
                continue;
            }
            if inner.rest().trim().is_empty() {
                break if elements.is_empty() {
                    Err(E::EmptyBlock.linked(&block_token_id))
                } else {
                    Ok(Some(Block {
                        elements,
                        owner: None,
                        breaker: None,
                        token: close(reader),
                    }))
                };
            } else {
                break Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner));
            }
        }
    }
}

impl Dissect<Block, Block> for Block {}
