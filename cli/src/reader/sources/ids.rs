use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Ids {
    index: usize,
}

impl Ids {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { index: 0 }))
    }
    pub fn get(&mut self) -> usize {
        self.index += 1;
        self.index - 1
    }
}
