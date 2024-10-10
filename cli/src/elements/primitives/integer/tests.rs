use crate::elements::{Element, InnersGetter, Integer};

impl InnersGetter for Integer {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}
