use crate::cli::{args::Argument, error::E};
use std::{io, io::Write, path::PathBuf};

const ARGS: [&str; 2] = ["--help", "-h"];

#[derive(Debug)]
pub struct Help {}

impl Argument<Help> for Help {
    fn read(args: &mut Vec<String>) -> Result<Option<Help>, E> {
        if let Some(first) = args.first() {
            if ARGS.contains(&first.as_str()) {
                if let Some(path) = args.get(1) {
                    let scenario = PathBuf::from(path);
                    if scenario.exists() {
                        Ok(Some(Help {}))
                    } else {
                        Err(E::FileNotExists(path.to_owned()))
                    }
                } else {
                    Err(E::NoPathToScenarioFile)
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    fn post(stdout: &mut io::Stdout) -> Result<(), io::Error> {
        stdout.write_all(format!("{} - shows help", ARGS.join(", ")).as_bytes())
    }
}
