mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

pub use cfm::*;
pub use declaration::*;
pub use expression::*;
pub use miscellaneous::*;
pub use root::*;
pub use statement::*;
pub use value::*;

use std::fmt;

#[derive(Debug, Clone)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
    Declaration(Declaration),
    Value(Value),
    ControlFlowModifier(ControlFlowModifier),
    Root(Root),
    Miscellaneous(Miscellaneous),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Statement(v) => v.to_string(),
                Self::Expression(v) => v.to_string(),
                Self::Declaration(v) => v.to_string(),
                Self::Value(v) => v.to_string(),
                Self::ControlFlowModifier(v) => v.to_string(),
                Self::Root(v) => v.to_string(),
                Self::Miscellaneous(v) => v.to_string(),
            }
        )
    }
}
