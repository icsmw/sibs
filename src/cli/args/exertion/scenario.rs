use crate::{
    cli::{
        args::{Action, ActionPinnedResult, Argument, Description},
        error::E,
    },
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
        if let Some(first) = args.first() {
            if ARGS.contains(&first.as_str()) {
                if let Some(path) = args.get(1) {
                    let scenario = PathBuf::from(path);
                    if scenario.exists() {
                        let _ = args.drain(0..=1);
                        Ok(Some(Box::new(Scenario { scenario })))
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
            desc: String::from("path to file - uses to define specific scenario file (*.sibs)"),
        }
    }
}

impl Action for Scenario {
    fn action<'a>(
        &'a self,
        _components: &'a [crate::elements::Component],
        _context: &'a mut crate::inf::Context,
    ) -> ActionPinnedResult {
        Box::pin(async move { Ok(AnyValue::new(self.scenario.clone())) })
    }
    fn key(&self) -> String {
        ARGS[0].to_owned()
    }
}
