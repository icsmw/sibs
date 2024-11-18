mod conflict;
mod link;
#[cfg(test)]
mod proptests;
mod read;
#[cfg(test)]
mod tests;

pub use read::*;
#[cfg(test)]
pub use tests::*;

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
