use crate::elements::{Call, Element, InnersGetter};

impl InnersGetter for Call {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.func.as_ref()]
    }
}
