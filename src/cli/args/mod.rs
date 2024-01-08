pub mod help;
pub mod target;

use crate::cli::{
    args::help::Help,
    error::E,
    reporter::{self, Reporter},
};
use std::{
    any::{Any, TypeId},
    collections::VecDeque,
    fmt::Debug,
};

pub struct Description {
    pub key: Vec<String>,
    pub desc: String,
}

pub trait Argument<T> {
    fn read(args: &mut Vec<String>) -> Result<Option<T>, E>
    where
        Self: Sized;
    fn desc() -> Description
    where
        Self: Sized;
}

pub trait DebugAny: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + Debug + 'static> DebugAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Arguments {
    pub arguments: Vec<Box<dyn DebugAny>>,
}

impl Arguments {
    pub fn new(args: &mut Vec<String>) -> Result<Self, E> {
        fn to_any<T: DebugAny + 'static>(
            entity: Result<Option<T>, E>,
        ) -> Result<Option<Box<dyn DebugAny>>, E> {
            entity.map(|o| o.map(|r| Box::new(r) as Box<dyn DebugAny>))
        }
        let mut all = VecDeque::from([
            to_any(target::Target::read(args))?,
            to_any(help::Help::read(args))?,
        ]);
        let mut arguments: Vec<Box<dyn DebugAny>> = vec![];
        while let Some(mut res) = all.pop_front() {
            if let Some(argument) = res.take() {
                arguments.push(argument);
            }
        }
        Ok(Self { arguments })
    }

    // pub fn find<T: 'static>(&mut self) -> Option<&mut T> {
    //     self.arguments
    //         .iter_mut()
    //         .find(|v| (*v.as_ref().as_any()).is::<T>())
    //         .and_then(|f| f.as_any_mut().downcast_mut::<T>())
    // }
}

impl reporter::Display for Arguments {
    fn display(&self, reporter: &mut Reporter) {
        print!("{}", reporter.offset());
        reporter.print(
            &[target::Target::desc(), help::Help::desc()]
                .iter()
                .map(|desc| format!("{}>>{}", desc.key.join(", "), desc.desc))
                .collect::<Vec<String>>()
                .join("\n"),
        );
    }
}
