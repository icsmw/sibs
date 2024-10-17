use crate::{
    elements::{compute::Operator, Compute, Element, ElementId},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Compute> for Compute {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Compute>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Compute);
        let Some(left) = Element::read(
            reader,
            &[
                ElementId::VariableName,
                ElementId::Function,
                ElementId::If,
                ElementId::Block,
                ElementId::Integer,
            ],
        )?
        else {
            return Ok(None);
        };
        reader.move_to().any();
        let Some(operator) =
            reader
                .move_to()
                .char(&[&chars::INC, &chars::DEC, &chars::DIV, &chars::MLT])
        else {
            return Ok(None);
        };
        let operator = match operator {
            chars::INC => Operator::Inc,
            chars::DEC => Operator::Dec,
            chars::DIV => Operator::Div,
            chars::MLT => Operator::Mlt,
            _ => {
                return Err(E::UnknownOperator(operator.to_string()).by_reader(reader));
            }
        };
        let Some(right) = Element::read(
            reader,
            &[
                ElementId::VariableName,
                ElementId::Function,
                ElementId::If,
                ElementId::Block,
                ElementId::Integer,
            ],
        )?
        else {
            return Err(E::NoRightSideAfterOperator.by_reader(reader));
        };
        Ok(Some(Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            token: close(reader),
        }))
    }
}

impl Dissect<Compute, Compute> for Compute {}
