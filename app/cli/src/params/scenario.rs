use crate::*;

const ARGS: [&str; 2] = ["--scenario", "-s"];

pub struct ScenarioParameter {}

impl Parameter for ScenarioParameter {
    fn keys() -> Vec<String> {
        ARGS.iter().map(|s| s.to_string()).collect()
    }
    fn desc() -> String {
        "Path to file - uses to define specific scenario file (*.sibs)".to_owned()
    }
    fn action(args: &mut Vec<String>) -> Option<Result<Action, E>> {
        let pos = args.iter().position(|arg| ARGS.contains(&arg.as_str()))?;
        if args.len() <= pos + 1 {
            return Some(Err(E::MissedPathWithScenario));
        }
        let filepath = args.remove(pos + 1);
        args.remove(pos);
        Some(Ok(Action::Scenario(ScenarioAction { filepath })))
    }
}
