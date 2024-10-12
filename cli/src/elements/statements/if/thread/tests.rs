use crate::elements::{Element, IfThread, InnersGetter};

impl InnersGetter for IfThread {
    fn get_inners(&self) -> Vec<&Element> {
        match self {
            Self::If(sub, block) => {
                vec![sub.as_ref(), block.as_ref()]
            }
            Self::Else(block) => vec![block.as_ref()],
        }
    }
}
