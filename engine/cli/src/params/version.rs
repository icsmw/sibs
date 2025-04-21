use crate::*;

const ARGS: [&str; 2] = ["--version", "-v"];

pub struct VersionParameter {}

impl Parameter for VersionParameter {
    fn keys() -> Vec<String> {
        ARGS.iter().map(|s| s.to_string()).collect()
    }
    fn desc() -> String {
        "Shows version of sibs".to_owned()
    }
    fn action(args: &mut Vec<String>) -> Option<Result<Action, E>> {
        let pos = args.iter().position(|arg| ARGS.contains(&arg.as_str()))?;
        args.remove(pos);
        Some(Ok(Action::Version(VersionAction {})))
    }
}
