use crate::{
    elements::{Element, ElementId, IfSubsequence},
    error::LinkedErr,
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<IfSubsequence> for IfSubsequence {
    fn try_dissect(reader: &mut Reader) -> Result<Option<IfSubsequence>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::IfSubsequence);
        let mut subsequence: Vec<Element> = Vec::new();
        while !reader.rest().trim().is_empty() {
            if subsequence.is_empty()
                || matches!(subsequence.last(), Some(Element::Combination(..)))
            {
                if let Some(el) = Element::include(
                    reader,
                    &[
                        ElementId::Boolean,
                        ElementId::Command,
                        ElementId::Comparing,
                        ElementId::Function,
                        ElementId::VariableName,
                        ElementId::Reference,
                        ElementId::IfCondition,
                    ],
                )? {
                    subsequence.push(el);
                } else {
                    break;
                }
            } else if let Some(el) = Element::include(reader, &[ElementId::Combination])? {
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
            Ok(Some(IfSubsequence {
                subsequence,
                token: close(reader),
            }))
        } else {
            Err(E::FailToReadConditions.linked(&close(reader)))
        }
    }
}

impl Dissect<IfSubsequence, IfSubsequence> for IfSubsequence {}
