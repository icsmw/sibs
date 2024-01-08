pub mod help;
pub mod target;

use crate::cli::{
    error::E,
    reporter::{self, Reporter},
};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
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
    pub arguments: HashMap<TypeId, Box<dyn DebugAny>>,
}

impl Arguments {
    pub fn new(args: &mut Vec<String>) -> Result<Self, E> {
        fn into<T: DebugAny + 'static>(entity: Option<T>) -> Option<(TypeId, Box<dyn DebugAny>)> {
            entity.map(|v| (TypeId::of::<T>(), Box::new(v) as Box<dyn DebugAny>))
        }
        let mut all = vec![
            into(target::Target::read(args)?),
            into(help::Help::read(args)?),
        ];
        let mut arguments: HashMap<TypeId, Box<dyn DebugAny>> = HashMap::new();
        while let Some(mut res) = all.pop() {
            if let Some((type_id, argument)) = res.take() {
                arguments.insert(type_id, argument);
            }
        }
        Ok(Self { arguments })
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.arguments
            .get(&TypeId::of::<T>())
            .and_then(|entity| entity.as_ref().as_any().downcast_ref())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.arguments
            .get_mut(&TypeId::of::<T>())
            .and_then(|entity| entity.as_mut().as_any_mut().downcast_mut())
    }

    pub fn remove<T: 'static>(&mut self) {
        let _ = self.arguments.remove(&TypeId::of::<T>());
    }
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
