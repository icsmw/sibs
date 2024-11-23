#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CommandPart {
    Open(Token),
    Literal(String),
    Expression(Node),
    Close(Token),
}

impl fmt::Display for CommandPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Open(tk) => tk.to_string(),
                Self::Close(tk) => tk.to_string(),
                Self::Literal(s) => s.to_owned(),
                Self::Expression(n) => format!("{} {n} {}", Kind::LeftBrace, Kind::RightBrace),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub nodes: Vec<CommandPart>,
    pub token: Token,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
