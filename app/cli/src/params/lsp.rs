use crate::*;

const ARGS: [&str; 2] = ["--lsp", "-lsp"];

pub struct LspParameter {}

impl Parameter for LspParameter {
    fn keys() -> Vec<String> {
        ARGS.iter().map(|s| s.to_string()).collect()
    }
    fn desc() -> String {
        "Run LSP server".to_owned()
    }
    fn action(args: &mut Vec<String>) -> Option<Result<Action, E>> {
        let pos = args.iter().position(|arg| ARGS.contains(&arg.as_str()))?;
        args.remove(pos);
        Some(Ok(Action::Lsp(LspAction::default())))
    }
}
