use crate::*;

const ARGS: [&str; 2] = ["--sessions", "-S"];

pub struct SessionsParameter {}

impl Parameter for SessionsParameter {
    fn keys() -> Vec<String> {
        ARGS.iter().map(|s| s.to_string()).collect()
    }
    fn desc() -> String {
        "Shows sessions of sibs".to_owned()
    }
    fn action(args: &mut Vec<String>) -> Option<Result<Action, E>> {
        let pos = args.iter().position(|arg| ARGS.contains(&arg.as_str()))?;
        args.remove(pos);
        Some(SessionsAction::new(if args.is_empty() {
            None
        } else {
            Some(args.remove(pos))
        })
        .map(Action::Sessions))
    }
}
