use crate::{
    elements::{Element, ElementRef, Subsequence},
    error::LinkedErr,
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Subsequence> for Subsequence {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Subsequence>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Subsequence);
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
        } else if reader.is_empty()
            || reader.next().is_word(&[
                words::IF,
                words::ELSE,
                &format!("{}", chars::OPEN_CURLY_BRACE),
            ])
        {
            Ok(Some(Subsequence {
                subsequence,
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Subsequence, Subsequence> for Subsequence {}
