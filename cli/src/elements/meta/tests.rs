use crate::elements::{Element, InnersGetter, Meta};

impl InnersGetter for Meta {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}
