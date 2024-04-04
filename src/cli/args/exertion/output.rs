use crate::{
    cli::{
        args::{Action, ActionPinnedResult, Argument, Description},
        error::E,
    },
    inf::{tracker, AnyValue},
};

const ARGS: [&str; 2] = ["--output", "-o"];

#[derive(Debug, Clone)]
pub struct Output {
    pub output: tracker::Output,
}

impl Argument for Output {
    fn key() -> String {
        ARGS[0].to_owned()
    }
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E> {
        if let Some(output) = Self::find_next_to(args, &ARGS)? {
            Ok(Some(Box::new(Output {
                output: tracker::Output::try_from(output)?,
            })))
        } else {
            Ok(None)
        }
    }
    fn desc() -> Description {
        Description {
            key: ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            desc: r#"Define a way of output:
        [b]--output progress[/b]  minimum output with progress (default);
        [b]--output logs[/b]      post logs into terminal;
        [b]--output none[/b]      no output;"#
                .to_string(),
        }
    }
}

impl Action for Output {
    fn action<'a>(
        &'a self,
        _components: &'a [crate::elements::Component],
        _context: &'a mut crate::inf::Context,
    ) -> ActionPinnedResult {
        Box::pin(async move { Ok(AnyValue::new(self.output.clone())) })
    }
    fn key(&self) -> String {
        ARGS[0].to_owned()
    }
}
