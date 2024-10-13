use crate::{
    elements::{Element, ElementId, IfCondition},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<IfCondition> for IfCondition {
    fn try_dissect(reader: &mut Reader) -> Result<Option<IfCondition>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::IfCondition);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            if let Some(el) = Element::include(&mut inner, &[ElementId::IfSubsequence])? {
                Ok(Some(IfCondition {
                    subsequence: Box::new(el),
                    token: close(reader),
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl Dissect<IfCondition, IfCondition> for IfCondition {}
