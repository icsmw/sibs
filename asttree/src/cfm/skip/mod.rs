#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum SkipTaskArgument {
    Value(LinkedNode),
    Any,
}

impl fmt::Display for SkipTaskArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Value(n) => n.to_string(),
                Self::Any => Kind::Star.to_string(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Skip {
    pub token: Token,
    pub args: Vec<SkipTaskArgument>,
    pub func: Box<LinkedNode>,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl SrcLinking for Skip {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.token, &self.close)
    }
    fn slink(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
}

impl fmt::Display for Skip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {}",
            self.token,
            self.open,
            Kind::LeftBracket,
            self.args
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            Kind::RightBracket,
            Kind::Comma,
            self.func,
            self.close
        )
    }
}

impl From<Skip> for Node {
    fn from(val: Skip) -> Self {
        Node::ControlFlowModifier(ControlFlowModifier::Skip(val))
    }
}
