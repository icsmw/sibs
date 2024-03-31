use crate::{
    cli::{
        args::{Argument, Description},
        error::E,
    },
    elements::Component,
    inf::{context::Context, tracker},
};

const ARGS: [&str; 2] = ["--output", "-o"];

#[derive(Debug, Clone)]
pub struct Output {
    pub output: tracker::Output,
}

impl Argument<Output> for Output {
    fn read(args: &mut Vec<String>) -> Result<Option<Output>, E> {
        if let Some(output) = Self::find_next_to(args, &ARGS)? {
            Ok(Some(Output {
                output: tracker::Output::try_from(output)?,
            }))
        } else {
            Ok(None)
        }
    }
    fn desc() -> Description {
        Description {
            key: ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            desc: String::from("output modes"),
            pairs: vec![
                (
                    String::from("--output progress"),
                    String::from("minimum output with progress (default);"),
                ),
                (
                    String::from("--output logs"),
                    String::from("post logs into terminal;"),
                ),
                (String::from("--output none"), String::from("no output;")),
            ],
        }
    }
    fn action(&mut self, _components: &[Component], _context: &mut Context) -> Result<(), E> {
        Ok(())
    }
}
