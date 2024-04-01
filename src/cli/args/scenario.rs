use crate::cli::{
    args::{Argument, Description},
    error::E,
};
use std::path::PathBuf;

const ARGS: [&str; 2] = ["--scenario", "-s"];

#[derive(Debug, Clone)]
pub struct Scenario {
    scenario: PathBuf,
}

impl Scenario {
    pub fn get(&self) -> PathBuf {
        self.scenario.clone()
    }
}

impl Argument<Scenario> for Scenario {
    fn read(args: &mut Vec<String>) -> Result<Option<Scenario>, E> {
        if let Some(first) = args.first() {
            if ARGS.contains(&first.as_str()) {
                if let Some(path) = args.get(1) {
                    let scenario = PathBuf::from(path);
                    if scenario.exists() {
                        let _ = args.drain(0..=1);
                        Ok(Some(Scenario { scenario }))
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
            pairs: vec![],
        }
    }
}
