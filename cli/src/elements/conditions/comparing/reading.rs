use crate::{
    elements::{conditions::Cmp, Comparing, Element, ElementRef},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Comparing> for Comparing {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Comparing>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Comparing);
        let left = if let Some(el) = Element::include(
            reader,
            &[
                ElementRef::VariableName,
                ElementRef::Command,
                ElementRef::Function,
                ElementRef::PatternString,
                ElementRef::Integer,
                ElementRef::Boolean,
            ],
        )? {
            Box::new(el)
        } else {
            return Ok(None);
        };
        let cmp = if let Some(word) = reader.move_to().expression(&[
            words::CMP_TRUE,
            words::CMP_FALSE,
            words::CMP_LBIG_INC,
            words::CMP_RBIG_INC,
            words::CMP_LBIG,
            words::CMP_RBIG,
        ]) {
            Cmp::from_str(&word)?
        } else {
            return Ok(None);
        };
        let right = if let Some(el) = Element::include(
            reader,
            &[
                ElementRef::VariableName,
                ElementRef::Command,
                ElementRef::Function,
                ElementRef::PatternString,
                ElementRef::Integer,
                ElementRef::Boolean,
            ],
        )? {
            Box::new(el)
        } else {
            return Ok(None);
        };
        Ok(Some(Comparing {
            left,
            cmp,
            right,
            token: close(reader),
        }))
    }
}

impl Dissect<Comparing, Comparing> for Comparing {}
