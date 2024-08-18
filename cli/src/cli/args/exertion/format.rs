use crate::{
    cli::{
        args::{Action, ActionPinnedResult, Argument, Description},
        error::E,
    },
    elements::Element,
    inf::{format_file, Value},
};
use std::path::PathBuf;

const ARGS: [&str; 2] = ["--format", "-f"];

#[derive(Debug, Clone)]
pub struct Format {
    filename: PathBuf,
}

impl Argument for Format {
    fn key() -> String {
        ARGS[0].to_owned()
    }
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E> {
        if let (true, path) = Self::with_next(args, &ARGS)? {
            let path = path.ok_or(E::NoPathToTargetFile(ARGS[0].to_owned()))?;
            let filename = PathBuf::from(&path);
            if filename.exists() {
                Ok(Some(Box::new(Format { filename })))
            } else {
                Err(E::FileNotExists(path.to_owned()))
            }
        } else {
            Ok(None)
        }
    }
    fn desc() -> Description {
        Description {
            key: ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            desc: String::from("path to file - format given *.sibs file"),
        }
    }
}

impl Action for Format {
    fn key(&self) -> String {
        ARGS[0].to_owned()
    }
    fn no_context(&self) -> bool {
        true
    }
    fn action<'a>(&'a self, _components: &'a [Element]) -> ActionPinnedResult {
        Box::pin(async move {
            format_file(&self.filename).await?;
            Ok(Value::empty())
        })
    }
}
