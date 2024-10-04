use crate::{
    cli::{
        args::{Action, ActionPinnedResult, Argument, Description},
        error::E,
    },
    elements::Element,
    inf::Value,
};

const ARGS: [&str; 1] = ["--trace"];

#[derive(Debug, Clone)]
pub struct Trace {
    pub state: bool,
}

impl Argument for Trace {
    fn key() -> String {
        ARGS[0].to_owned()
    }
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E> {
        if Self::find(args, &ARGS)? {
            Ok(Some(Box::new(Self { state: true })))
        } else {
            Ok(None)
        }
    }
    fn desc() -> Description {
        Description {
            key: ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            desc: String::from("Include into logs trace messages"),
        }
    }
}

impl Action for Trace {
    fn action<'a>(&'a self, _components: &'a [Element]) -> ActionPinnedResult<'a> {
        Box::pin(async move { Ok(Value::bool(self.state)) })
    }
    fn key(&self) -> String {
        ARGS[0].to_owned()
    }
}
