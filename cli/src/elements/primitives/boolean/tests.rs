use crate::elements::{Boolean, Element, InnersGetter};

impl InnersGetter for Boolean {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}
