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

pub(crate) use lexer::*;
pub(crate) use uuid::Uuid;

#[cfg(feature = "proptests")]
pub const PROPTEST_DEEP_FACTOR: u8 = 5;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
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

impl Node {
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::Statement(n) => n.uuid(),
            Self::Expression(n) => n.uuid(),
            Self::Declaration(n) => n.uuid(),
            Self::Value(n) => n.uuid(),
            Self::ControlFlowModifier(n) => n.uuid(),
            Self::Root(n) => n.uuid(),
            Self::Miscellaneous(n) => n.uuid(),
        }
    }
}
