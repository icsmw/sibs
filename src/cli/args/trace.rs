use crate::{
    cli::{
        args::{Argument, Description},
        error::E,
    },
    elements::Component,
    inf::context::Context,
};

const ARGS: [&str; 1] = ["--trace"];

#[derive(Debug, Clone)]
pub struct Trace {
    pub state: bool,
}

impl Argument<Trace> for Trace {
    fn read(args: &mut Vec<String>) -> Result<Option<Trace>, E> {
        Self::has(args, &ARGS).map(|state| Some(Self { state }))
    }
    fn desc() -> Description {
        Description {
            key: ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            desc: String::from("Include into logs trace messages"),
            pairs: vec![],
        }
    }
    fn action(&mut self, _components: &[Component], _context: &mut Context) -> Result<(), E> {
        Ok(())
    }
}
