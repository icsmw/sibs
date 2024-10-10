use crate::elements::{Element, InnersGetter, Range};

impl InnersGetter for Range {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.from.as_ref(), self.to.as_ref()]
    }
}
