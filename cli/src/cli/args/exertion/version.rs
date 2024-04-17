use crate::{
    cli::{
        args::{Action, ActionPinnedResult, Argument, Description},
        error::E,
    },
    elements::Element,
    inf::AnyValue,
};

const ARGS: [&str; 2] = ["--version", "-v"];
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct Version {}

impl Argument for Version {
    fn key() -> String {
        ARGS[0].to_owned()
    }
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E> {
        if Self::find(args, &ARGS)? {
            Ok(Some(Box::new(Version {})))
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
}

impl Action for Version {
    fn key(&self) -> String {
        ARGS[0].to_owned()
    }
    fn no_context(&self) -> bool {
        true
    }
    fn action<'a>(&'a self, _components: &'a [Element]) -> ActionPinnedResult {
        Box::pin(async move {
            println!("{VERSION}");
            Ok(AnyValue::new(()))
        })
    }
}
