use crate::{
    cli::{
        args::{Action, ActionPinnedResult, Argument, Description},
        error::E,
    },
    elements::Component,
    inf::{context::Context, format_file, AnyValue},
};
use std::path::PathBuf;

const ARGS: [&str; 2] = ["--format", "-f"];

#[derive(Debug, Clone)]
pub struct Format {
    target: PathBuf,
}

impl Argument for Format {
    fn key() -> String {
        ARGS[0].to_owned()
    }
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E> {
        if let Some(first) = args.first() {
            if ARGS.contains(&first.as_str()) {
                if let Some(path) = args.get(1) {
                    let target = PathBuf::from(path);
                    if target.exists() {
                        let _ = args.drain(0..=1);
                        Ok(Some(Box::new(Format { target })))
                    } else {
                        Err(E::FileNotExists(path.to_owned()))
                    }
                } else {
                    Err(E::NoPathToTargetFile(ARGS[0].to_owned()))
                }
            } else {
                Ok(None)
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
    fn action<'a>(
        &'a self,
        _components: &'a [Component],
        _cx: &'a mut Context,
    ) -> ActionPinnedResult {
        Box::pin(async move {
            format_file(&self.target).await?;
            Ok(AnyValue::new(()))
        })
    }
}
