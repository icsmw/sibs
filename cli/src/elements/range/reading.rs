use crate::{
    elements::{Element, ElementId, Range},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Range> for Range {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Range>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Range);
        let Some(from) = Element::include(reader, &[ElementId::VariableName, ElementId::Integer])?
        else {
            return Ok(None);
        };
        if reader.move_to().word_any(&[words::RANGE]).is_none() {
            return Ok(None);
        }
        let Some(to) = Element::include(reader, &[ElementId::VariableName, ElementId::Integer])?
        else {
            return Err(E::NoEndRangeBorder.by_reader(reader));
        };
        Ok(Some(Self {
            from: Box::new(from),
            to: Box::new(to),
            token: close(reader),
        }))
    }
}

impl Dissect<Range, Range> for Range {}
