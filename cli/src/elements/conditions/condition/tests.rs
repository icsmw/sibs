use crate::elements::{Condition, Element, InnersGetter};

impl InnersGetter for Condition {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.subsequence.as_ref()]
    }
}
