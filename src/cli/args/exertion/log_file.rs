use std::path::PathBuf;

use crate::{
    cli::{
        args::{Action, ActionPinnedResult, Argument, Description},
        error::E,
    },
    inf::{tracker, AnyValue},
};

const ARGS: [&str; 2] = ["--logs", "-l"];

#[derive(Debug, Clone)]
pub struct LogFile {
    pub file: String,
}

impl Argument for LogFile {
    fn key() -> String {
        ARGS[0].to_owned()
    }
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E> {
        if let Some(file) = Self::find_next_to(args, &ARGS).map(|file| file.map(|file| file))? {
            Ok(Some(Box::new(Self { file })))
        } else {
            Ok(None)
        }
    }
    fn desc() -> Description {
        Description {
            key: ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            desc: String::from("saves logs into given file"),
            pairs: vec![],
        }
    }
}

impl Action for LogFile {
    fn action<'a>(
        &'a self,
        _components: &'a [crate::elements::Component],
        _context: &'a mut crate::inf::Context,
    ) -> ActionPinnedResult {
        Box::pin(async move { Ok(AnyValue::new(PathBuf::from(&self.file))) })
    }
    fn key(&self) -> String {
        ARGS[0].to_owned()
    }
}
