use crate::elements::{Accessor, Element, InnersGetter};

impl InnersGetter for Accessor {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.index.as_ref()]
    }
}
