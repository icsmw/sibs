use crate::elements::{Element, IfCondition, InnersGetter};
impl InnersGetter for IfCondition {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.subsequence.as_ref()]
    }
}
