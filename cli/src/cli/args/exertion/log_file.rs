use std::path::PathBuf;

use crate::{
    cli::{
        args::{Action, ActionPinnedResult, Argument, Description},
        error::E,
    },
    elements::Element,
    inf::AnyValue,
};

const ARGS: [&str; 2] = ["--logs", "-l"];

#[derive(Debug, Clone)]
pub struct LogFile {
    pub filename: String,
}

impl Argument for LogFile {
    fn key() -> String {
        ARGS[0].to_owned()
    }
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E> {
        if let (true, filename) = Self::with_next(args, &ARGS)? {
            let filename = filename.ok_or(E::NeedsArgumentAfter(ARGS[0].to_owned()))?;
            Ok(Some(Box::new(Self { filename })))
        } else {
            Ok(None)
        }
    }
    fn desc() -> Description {
        Description {
            key: ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            desc: String::from("saves logs into given file"),
        }
    }
}

impl Action for LogFile {
    fn action<'a>(&'a self, _components: &'a [Element]) -> ActionPinnedResult {
        Box::pin(async move { Ok(AnyValue::new(PathBuf::from(&self.filename))) })
    }
    fn key(&self) -> String {
        ARGS[0].to_owned()
    }
}
