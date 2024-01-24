pub mod import;
pub mod os;

use crate::{
    error::E,
    inf::any::AnyValue,
    inf::context::Context,
    reader::{self, entry::Function},
};
use std::{future::Future, pin::Pin};

pub type ExecutorPinnedResult<'a> = Pin<Box<dyn Future<Output = ExecutorResult> + 'a>>;
pub type ExecutorResult = Result<Option<AnyValue>, E>;
pub type ExecutorFn = for<'a> fn(&'a mut Function, &'a mut Context) -> ExecutorPinnedResult<'a>;

pub trait Executor {
    fn from<'a>(function: &'a mut Function, cx: &'a mut Context) -> ExecutorPinnedResult<'a>;
    fn get_name() -> String;
}

pub fn register(cx: &mut Context) -> Result<(), reader::error::E> {
    cx.add_fn(
        import::Import::get_name(),
        <import::Import as Executor>::from,
    )?;
    cx.add_fn(os::Os::get_name(), <os::Os as Executor>::from)?;
    Ok(())
}

// use std::{collections::HashMap, future::Future, path::PathBuf, pin::Pin};

// type CommandExecution =
//     for<'a> fn(
//         &'a mut Function,
//         &'a mut Store,
//     ) -> Pin<Box<dyn Future<Output = Result<Option<AnyValue>, E>> + 'a>>;

// struct Store {
//     pub map: HashMap<String, CommandExecution>,
//     pub cwd: Option<PathBuf>,
// }

// impl Store {
//     pub fn add(&mut self, name: String, func: CommandExecution) {
//         self.map.insert(name, func);
//     }

//     pub async fn run(&mut self, name: String, func: &mut Function) -> Result<Option<AnyValue>, E> {
//         if let Some(f) = self.map.get(&name) {
//             f(func, self).await
//         } else {
//             Ok(None)
//         }
//     }
// }
// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum Error {
//     #[error("No arguments; path required")]
//     NoArguments,
//     #[error("Only one argument is required: path")]
//     InvalidNumberOfArguments,
//     #[error("As path expected string value")]
//     InvalidPathArgument,
//     #[error("File {0} doesn't exist")]
//     NoFile(String),
//     #[error("Import action required CWD")]
//     NoCurrentWorkingFolder,
// }

// impl From<Error> for E {
//     fn from(e: Error) -> Self {
//         E {
//             sig: format!("@test"),
//             msg: e.to_string(),
//         }
//     }
// }
// struct A {}

// impl A {
//     fn executor<'a>(
//         function: &'a mut Function,
//         cx: &'a mut Store,
//     ) -> Pin<Box<dyn Future<Output = Result<Option<AnyValue>, E>> + 'a>> {
//         Box::pin(async {
//             let cwd = cx.cwd.as_ref().ok_or(Error::NoCurrentWorkingFolder)?;
//             let args = function.args.as_mut().ok_or(Error::NoArguments)?;
//             if args.args.len() != 1 {
//                 Err(Error::InvalidNumberOfArguments)?;
//             }
//             let mut path = if let (_, Argument::String(value)) = &args.args[0] {
//                 PathBuf::from(value)
//             } else {
//                 Err(Error::InvalidPathArgument)?
//             };
//             if path.is_relative() {
//                 path = cwd.join(path);
//             }
//             if !path.exists() {
//                 Err(Error::NoFile(path.to_string_lossy().to_string()))?;
//             }
//             Ok(Some(AnyValue::new(path)))
//         })
//     }
// }

// struct B {}

// impl B {
//     fn executor<'a>(
//         function: &'a mut Function,
//         cx: &'a mut Store,
//     ) -> Pin<Box<dyn Future<Output = Result<Option<AnyValue>, E>> + 'a>> {
//         Box::pin(async { Ok(None) })
//     }
// }

// fn test() {
//     let mut s = Store {
//         map: HashMap::new(),
//         cwd: None,
//     };
//     s.add("name".to_string(), A::executor);
//     s.add("name".to_string(), B::executor);
// }
