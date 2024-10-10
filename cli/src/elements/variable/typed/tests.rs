use crate::elements::{Element, InnersGetter, VariableType};

impl InnersGetter for VariableType {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}
