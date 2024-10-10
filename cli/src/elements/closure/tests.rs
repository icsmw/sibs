use crate::elements::{Closure, Element, InnersGetter};

impl InnersGetter for Closure {
    fn get_inners(&self) -> Vec<&Element> {
        [self.args.iter().collect(), vec![self.block.as_ref()]].concat()
    }
}
