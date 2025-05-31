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
    fn action(_args: &mut Vec<String>) -> Option<Result<Action, E>> {
        Some(Ok(Action::Lsp(LspAction::default())))
    }
}
