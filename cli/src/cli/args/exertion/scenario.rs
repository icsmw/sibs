use crate::{
    cli::{
        args::{Action, ActionPinnedResult, Argument, Description},
        error::E,
    },
    elements::Element,
    inf::AnyValue,
};
use std::path::PathBuf;

const ARGS: [&str; 2] = ["--scenario", "-s"];

#[derive(Debug, Clone)]
pub struct Scenario {
    scenario: PathBuf,
}

impl Argument for Scenario {
    fn key() -> String {
        ARGS[0].to_owned()
    }
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E> {
        if let (true, filename) = Self::with_next(args, &ARGS)? {
            let filename = filename.ok_or(E::NoPathToTargetFile(ARGS[0].to_owned()))?;
            let scenario = PathBuf::from(&filename);
            if scenario.exists() {
                Ok(Some(Box::new(Scenario { scenario })))
            } else {
                Err(E::FileNotExists(filename.to_owned()))
            }
        } else {
            Ok(None)
        }
    }
    fn desc() -> Description {
        Description {
            key: ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            desc: String::from("path to file - uses to define specific scenario file (*.sibs)"),
        }
    }
}

impl Action for Scenario {
    fn action<'a>(
        &'a self,
        _components: &'a [Element],
        _context: &'a mut crate::inf::Context,
    ) -> ActionPinnedResult {
        Box::pin(async move { Ok(AnyValue::new(self.scenario.clone())) })
    }
    fn key(&self) -> String {
        ARGS[0].to_owned()
    }
}
