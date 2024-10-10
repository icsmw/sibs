use crate::elements::{Element, Incrementer, InnersGetter};

impl InnersGetter for Incrementer {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.variable.as_ref(), self.right.as_ref()]
    }
}
