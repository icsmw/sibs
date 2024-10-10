use crate::elements::{Combination, Element, InnersGetter};

impl InnersGetter for Combination {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}
