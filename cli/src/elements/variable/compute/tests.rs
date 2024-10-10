use crate::elements::{Compute, Element, InnersGetter};

impl InnersGetter for Compute {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}
