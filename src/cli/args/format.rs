use crate::{
    cli::{
        args::{Argument, Description},
        error::E,
    },
    elements::Component,
    inf::{context::Context, format_file},
};
use std::path::PathBuf;

const ARGS: [&str; 2] = ["--format", "-f"];

#[derive(Debug, Clone)]
pub struct Format {
    target: PathBuf,
}

impl Argument<Format> for Format {
    fn read(args: &mut Vec<String>) -> Result<Option<Format>, E> {
        if let Some(first) = args.first() {
            if ARGS.contains(&first.as_str()) {
                if let Some(path) = args.get(1) {
                    let target = PathBuf::from(path);
                    if target.exists() {
                        let _ = args.drain(0..=1);
                        Ok(Some(Format { target }))
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
            pairs: vec![],
        }
    }
    fn no_context(&self) -> bool {
        true
    }
    async fn action(&mut self, _components: &[Component], _context: &mut Context) -> Result<(), E> {
        format_file(&self.target).await?;
        Ok(())
    }
}
