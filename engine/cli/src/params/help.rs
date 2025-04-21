use crate::*;

const ARGS: [&str; 2] = ["--help", "-h"];

pub struct HelpParameter {}

impl Parameter for HelpParameter {
    fn keys() -> Vec<String> {
        ARGS.iter().map(|s| s.to_string()).collect()
    }
    fn desc() -> String {
        "Shows help or available options and components. To get help for component use \"--help component_name [task_name]\".".to_owned()
    }
    fn action(args: &mut Vec<String>) -> Option<Result<Action, E>> {
        let pos = args.iter().position(|arg| ARGS.contains(&arg.as_str()))?;
        args.remove(pos);
        Some(Ok(Action::Help(HelpAction {})))
    }
}
