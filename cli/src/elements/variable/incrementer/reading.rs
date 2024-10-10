use crate::{
    elements::{incrementer::Operator, Element, ElementRef, Incrementer},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Incrementer> for Incrementer {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Incrementer>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Incrementer);
        let Some(variable) = Element::include(reader, &[ElementRef::VariableName])? else {
            return Ok(None);
        };
        reader.move_to().any();
        let Some(operator) = reader.move_to().word_any(&[words::INC_BY, words::DEC_BY]) else {
            return Ok(None);
        };
        let operator = match operator.as_str() {
            words::INC_BY => Operator::Inc,
            words::DEC_BY => Operator::Dec,
            _ => {
                return Err(E::UnknownOperator(operator.to_string()).by_reader(reader));
            }
        };
        let Some(right) = Element::include(
            reader,
            &[
                ElementRef::VariableName,
                ElementRef::Function,
                ElementRef::If,
                ElementRef::Block,
                ElementRef::Integer,
            ],
        )?
        else {
            return Err(E::NoRightSideAfterOperator.by_reader(reader));
        };
        Ok(Some(Self {
            variable: Box::new(variable),
            operator,
            right: Box::new(right),
            token: close(reader),
        }))
    }
}

impl Dissect<Incrementer, Incrementer> for Incrementer {}
