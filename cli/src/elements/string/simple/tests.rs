use crate::elements::{Element, InnersGetter, SimpleString};
impl InnersGetter for SimpleString {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}
