pub mod help;
pub mod target;

use crate::cli::{
    error::E,
    reporter::{self, Description},
};
use std::{any::Any, collections::VecDeque};

pub trait Argument<T> {
    fn read(args: &mut Vec<String>) -> Result<Option<T>, E>
    where
        Self: Sized;
    fn desc() -> Description
    where
        Self: Sized;
}

pub struct Arguments {
    arguments: Vec<Box<dyn Any>>,
}

impl Arguments {
    pub fn new(args: &mut Vec<String>) -> Result<Self, E> {
        fn to_any<T: 'static>(entity: Result<Option<T>, E>) -> Result<Option<Box<dyn Any>>, E> {
            entity.map(|o| o.map(|r| Box::new(r) as Box<dyn Any>))
        }
        let mut all = VecDeque::from([
            to_any(target::Target::read(args))?,
            to_any(help::Help::read(args))?,
        ]);
        let mut arguments: Vec<Box<dyn Any>> = vec![];
        while let Some(mut res) = all.pop_front() {
            if let Some(argument) = res.take() {
                arguments.push(argument);
            }
        }
        Ok(Self { arguments })
    }

    pub fn take<T: 'static>(&mut self) -> Option<Box<T>> {
        if let Some(i) = self
            .arguments
            .iter()
            .position(|entity| entity.as_ref().downcast_ref::<T>().is_some())
        {
            Some(self.arguments.remove(i).downcast::<T>().ok()?)
        } else {
            None
        }
    }

    pub fn desc() {
        reporter::desc(vec![target::Target::desc(), help::Help::desc()]);
    }
}
