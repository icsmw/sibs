mod variable;

pub use variable::*;

pub enum Expression {
    // Call(Call),
    // Accessor(Accessor),
    // Comparing(Comparing),
    // Combination(Combination),
    // Subsequence(Subsequence),
    // Condition(Condition),
    // Compute(Compute),
    // Optional(Optional),
    // PatternString(PatternString),
    // Range(Range),
    Variable(Variable),
}
