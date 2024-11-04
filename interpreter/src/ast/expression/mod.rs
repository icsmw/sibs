mod conflict;
mod interest;
mod read;

mod accessor;
mod binary_exp;
mod binary_op;
mod call;
mod command;
mod comparison;
mod comparison_op;
mod comparison_seq;
mod compound_assignments;
mod compound_assignments_op;
mod condition;
mod function_call;
mod logical_op;
mod range;
mod task_call;
mod variable;

pub use accessor::*;
pub use binary_exp::*;
pub use binary_op::*;
pub use call::*;
pub use command::*;
pub use comparison::*;
pub use comparison_op::*;
pub use comparison_seq::*;
pub use compound_assignments::*;
pub use compound_assignments_op::*;
pub use condition::*;
pub use function_call::*;
pub use logical_op::*;
pub use range::*;
pub use task_call::*;
pub use variable::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Expression {
    Call(Call),
    Accessor(Accessor),
    Condition(Condition),
    /// &&, ||
    LogicalOp(LogicalOp),
    /// ==, >=, <=, !=, <, >
    ComparisonOp(ComparisonOp),
    /// x < y, x <= y, x == y, etc.
    Comparison(Comparison),
    ComparisonSeq(ComparisonSeq),
    Range(Range),
    Variable(Variable),
    /// 1 + 2, 1 / 2, (x + 2) / 3, etc.
    BinaryExp(BinaryExp),
    /// +, -, *, /
    BinaryOp(BinaryOp),
    FunctionCall(FunctionCall),
    /// x += 1, x -= 1, x *= 2, x /= 2
    CompoundAssignments(CompoundAssignments),
    /// +=, -=, *=, \=
    CompoundAssignmentsOp(CompoundAssignmentsOp),
    Command(Command),
    TaskCall(TaskCall),
}
