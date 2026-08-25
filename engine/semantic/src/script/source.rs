use crate::ScriptError;
use lexer::*;
use parser::*;
use std::{fmt, path::PathBuf};

#[derive(Debug, Clone)]
pub enum ScriptSource {
    File(PathBuf),
    Text(String),
}

impl ScriptSource {
    pub fn file<P: Into<PathBuf>>(path: P) -> Self {
        Self::File(path.into())
    }

    pub fn text<S: ToString>(content: S) -> Self {
        Self::Text(content.to_string())
    }

    pub(crate) fn parser(&self, resilience: bool) -> Result<Parser, ScriptError> {
        match self {
            Self::File(path) => Ok(Parser::new(path, resilience)?),
            Self::Text(content) => {
                let mut lx = Lexer::new(content, 0);
                Ok(Parser::unbound(
                    lx.read()?.tokens,
                    &lx.uuid,
                    content,
                    resilience,
                ))
            }
        }
    }
}

impl fmt::Display for ScriptSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(path) => write!(f, "{}", path.to_string_lossy()),
            Self::Text(..) => write!(f, "text script"),
        }
    }
}
