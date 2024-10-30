mod conflict;
mod interest;
mod read;
mod variable;

pub use variable::*;

use std::fmt;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone")]
#[derive(Debug, Clone)]
pub enum Expression {
    // Call(Call),
    // Accessor(Accessor),
    // Comparing(Comparing),
    // Combination(Combination),
    // Subsequence(Subsequence),
    // Condition(Condition),
    // Compute(Compute),
    // Optional(Optional),
    // Range(Range),
    Variable(Variable),
}

impl fmt::Display for ExpressionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Variable => "Expression::Variable",
            }
        )
    }
}
