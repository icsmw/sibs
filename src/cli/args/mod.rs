pub mod help;
pub mod target;
pub mod version;

use crate::{
    cli::error::E,
    inf::{
        any::DebugAny,
        context::Context,
        term::{self, Term},
    },
    reader::entry::Component,
};
use std::{any::TypeId, collections::HashMap, fmt::Debug};

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
    fn action(&mut self, _components: &[Component], _context: &mut Context) -> Result<(), E> {
        Ok(())
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
            into(version::Version::read(args)?),
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

    pub fn has<T: 'static>(&self) -> bool {
        self.arguments.get(&TypeId::of::<T>()).is_some()
    }

    pub fn remove<T: 'static>(&mut self) {
        let _ = self.arguments.remove(&TypeId::of::<T>());
    }
}

impl term::Display for Arguments {
    fn display(&self, term: &mut Term) {
        term.print_fmt(
            &[target::Target::desc(), help::Help::desc()]
                .iter()
                .map(|desc| format!("{}>>{}", desc.key.join(", "), desc.desc))
                .collect::<Vec<String>>(),
        );
    }
}
