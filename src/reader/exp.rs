use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Map {
    map: HashMap<usize, (usize, usize)>,
    index: usize,
}

pub struct Child<'a> {
    parent: &'a mut Parent,
}
impl<'a> Child<'a> {
    pub fn new(parent: &'a mut Parent) -> Self {
        Self { parent }
    }
}

pub struct Parent {
    map: Rc<RefCell<Map>>,
}

impl Parent {
    pub fn new(map: Rc<RefCell<Map>>) -> Self {
        Self { map }
    }
    pub fn child(&mut self) -> Child<'_> {
        Child::new(self)
    }
}

// pub struct ParentA<'a> {
//     map: &'a mut Map,
//     storage: Option<Box<dyn std::any::Any>>,
// }

// impl<'a> ParentA<'a> {
//     pub fn new(map: &'a mut Map) -> Self {
//         Self { map, storage: None }
//     }
//     pub fn child(&mut self) -> Box<&mut Child<'a>> {
//         self.storage = Some(Box::new(Child { parent: self }));
//         self.storage.as_mut().unwrap() as Box<&mut Child<'a>>
//     }
// }
