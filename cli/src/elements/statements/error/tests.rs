use crate::elements::{Element, Error, InnersGetter};

impl InnersGetter for Error {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.output.as_ref()]
    }
}
