use crate::elements::{Element, InnersGetter, VariableDeclaration};

impl InnersGetter for VariableDeclaration {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.variable.as_ref(), self.declaration.as_ref()]
    }
}
