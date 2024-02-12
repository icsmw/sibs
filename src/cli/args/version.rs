use crate::{
    cli::{
        args::{Argument, Description},
        error::E,
    },
    entry::Component,
    inf::context::Context,
};

const ARGS: [&str; 2] = ["--version", "-v"];

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Version {}

impl Argument<Version> for Version {
    fn read(args: &mut Vec<String>) -> Result<Option<Version>, E> {
        if let Some(first) = args.first() {
            if ARGS.contains(&first.as_str()) {
                let _ = args.remove(0);
                Ok(Some(Version {}))
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
            desc: String::from("show version of sibs"),
        }
    }
    fn action(&mut self, _components: &[Component], _context: &mut Context) -> Result<(), E> {
        println!("{VERSION}");
        Ok(())
    }
}
