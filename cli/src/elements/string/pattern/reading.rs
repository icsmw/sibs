use crate::{
    elements::{string, ElementRef, PatternString},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<PatternString> for PatternString {
    fn try_dissect(reader: &mut Reader) -> Result<Option<PatternString>, LinkedErr<E>> {
        if let Some((_, elements, token)) =
            string::read(reader, chars::QUOTES, ElementRef::PatternString)?
        {
            Ok(Some(PatternString { elements, token }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<PatternString, PatternString> for PatternString {}
