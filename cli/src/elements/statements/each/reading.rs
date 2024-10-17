use crate::{
    elements::{Each, Element, ElementId},
    error::LinkedErr,
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};
use tokio_util::sync::CancellationToken;

impl TryDissect<Each> for Each {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Each>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Each);
        if reader.move_to().word(&[words::EACH]).is_some() {
            let (variable, input) = if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                let variable = if let Some(variable) =
                    Element::read(&mut inner, &[ElementId::VariableName])?
                {
                    if inner.move_to().char(&[&chars::SEMICOLON]).is_none() {
                        return Err(E::MissedSemicolon.linked(&inner.token()?.id));
                    }
                    variable
                } else {
                    return Err(E::NoLoopVariable.linked(&inner.token()?.id));
                };
                let input = if let Some(el) =
                    Element::read(&mut inner, &[ElementId::Function, ElementId::VariableName])?
                {
                    Box::new(el)
                } else {
                    Err(E::NoLoopInput.by_reader(&inner))?
                };
                (variable, input)
            } else {
                return Err(E::NoLoopInitialization.linked(&reader.token()?.id));
            };
            let Some(mut block) = Element::read(reader, &[ElementId::Block])? else {
                Err(E::NoGroup.by_reader(reader))?
            };
            if let Element::Block(block, _) = &mut block {
                block.set_owner(ElementId::Each);
                block.set_breaker(CancellationToken::new());
            }
            Ok(Some(Each {
                input,
                variable: Box::new(variable),
                block: Box::new(block),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Each, Each> for Each {}
