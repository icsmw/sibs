pub mod help;
pub mod log_file;
pub mod output;
pub mod target;
pub mod trace;
pub mod version;

use crate::{
    cli::error::E,
    entry::Component,
    inf::{
        any::DebugAny,
        context::Context,
        term::{self, Term},
    },
};
use std::{any::TypeId, collections::HashMap, fmt::Debug};

pub struct Description {
    pub key: Vec<String>,
    pub desc: String,
    pub pairs: Vec<(String, String)>,
}

pub trait Argument<T> {
    fn find_next_to(args: &mut Vec<String>, targets: &[&str]) -> Result<Option<String>, E> {
        if let Some(position) = args.iter().position(|arg| targets.contains(&arg.as_str())) {
            if position == args.len() - 1 {
                Err(E::InvalidRequestAfter(targets.join(", ")))?;
            }
            let _ = args.remove(position);
            Ok(Some(args.remove(position)))
        } else {
            Ok(None)
        }
    }
    fn find_prev_to_opt(
        args: &mut Vec<String>,
        targets: &[&str],
    ) -> Result<Option<Option<String>>, E> {
        if let Some(position) = args.iter().position(|arg| targets.contains(&arg.as_str())) {
            if position == 0 {
                Ok(Some(None))
            } else {
                let arg = args.remove(position - 1);
                args.remove(position - 1);
                Ok(Some(Some(arg)))
            }
        } else {
            Ok(None)
        }
    }
    fn has(args: &mut Vec<String>, targets: &[&str]) -> Result<bool, E> {
        if let Some(position) = args.iter().position(|arg| targets.contains(&arg.as_str())) {
            let _ = args.remove(position);
            Ok(true)
        } else {
            Ok(false)
        }
    }
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
            into(output::Output::read(args)?),
            into(log_file::LogFile::read(args)?),
            into(trace::Trace::read(args)?),
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
        self.arguments.contains_key(&TypeId::of::<T>())
    }
}

impl term::Display for Arguments {
    fn display(&self, term: &mut Term) {
        term.print_fmt(
            &[
                target::Target::desc(),
                help::Help::desc(),
                trace::Trace::desc(),
                output::Output::desc(),
                log_file::LogFile::desc(),
                version::Version::desc(),
            ]
            .iter()
            .flat_map(|desc| {
                [
                    vec![format!("{}>>{}", desc.key.join(", "), desc.desc)],
                    desc.pairs
                        .iter()
                        .map(|(key, value)| format!("{}>>{}", key, value))
                        .collect::<Vec<String>>(),
                ]
                .concat()
            })
            .collect::<Vec<String>>(),
        );
    }
}
