use crate::{
    elements::{Conclusion, Element, ElementRef},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Conclusion> for Conclusion {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Conclusion>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Conclusion);
        let mut subsequence: Vec<Element> = Vec::new();
        while !reader.rest().trim().is_empty() {
            if subsequence.is_empty()
                || matches!(subsequence.last(), Some(Element::Combination(..)))
            {
                if let Some(el) =
                    Element::include(reader, &[ElementRef::Comparing, ElementRef::Condition])?
                {
                    subsequence.push(el);
                } else {
                    break;
                }
            } else if let Some(el) = Element::include(reader, &[ElementRef::Combination])? {
                subsequence.push(el);
            } else {
                break;
            }
        }
        if subsequence.is_empty() {
            Ok(None)
        } else if reader.is_empty() || reader.next().is_word(&[&format!("{}", chars::SEMICOLON)]) {
            Ok(Some(Conclusion {
                subsequence,
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Conclusion, Conclusion> for Conclusion {}
