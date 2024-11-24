mod accessor;
mod binary_exp;
mod binary_exp_group;
mod binary_exp_seq;
mod binary_op;
mod call;
mod command;
mod comparison;
mod comparison_group;
mod comparison_op;
mod comparison_seq;
mod compound_assignments;
mod compound_assignments_op;
mod function_call;
mod logical_op;
mod range;
mod task_call;
mod variable;

pub use accessor::*;
pub use binary_exp::*;
pub use binary_exp_group::*;
pub use binary_exp_seq::*;
pub use binary_op::*;
pub use call::*;
pub use command::*;
pub use comparison::*;
pub use comparison_group::*;
pub use comparison_op::*;
pub use comparison_seq::*;
pub use compound_assignments::*;
pub use compound_assignments_op::*;
pub use function_call::*;
pub use logical_op::*;
pub use range::*;
pub use task_call::*;
pub use variable::*;

use crate::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Expression {
    /// .as_str(), .is_err(), .results::is_err(args) call of function after expression
    Call(Call),
    /// [1], [n], [get_index()], access to indexed value
    Accessor(Accessor),
    /// &&, ||
    LogicalOp(LogicalOp),
    /// ==, >=, <=, !=, <, >
    ComparisonOp(ComparisonOp),
    /// x < y, x <= y, x == y, etc.
    Comparison(Comparison),
    /// (x), (x < y), (x < y && c > t), etc, but always in (...)
    ComparisonGroup(ComparisonGroup),
    /// t == v || (x < y), (x < y && c > t) || u != p, etc
    ComparisonSeq(ComparisonSeq),
    /// 1..3, n..4, n..m, etc
    Range(Range),
    /// a, b, c, x, y, etc
    Variable(Variable),
    /// (x + 2) / 3, (a + b) * x / t etc.
    BinaryExpSeq(BinaryExpSeq),
    /// 1 + 2, a + 2 etc, primitive expression
    BinaryExp(BinaryExp),
    /// (1 + 2), (a + b), etc, but always in (...)
    BinaryExpGroup(BinaryExpGroup),
    /// +, -, *, /
    BinaryOp(BinaryOp),
    /// func(), get_os(), mod::func(arg, arg) etc.
    FunctionCall(FunctionCall),
    /// x += 1, x -= 1, x *= 2, x /= 2
    CompoundAssignments(CompoundAssignments),
    /// +=, -=, *=, \=
    CompoundAssignmentsOp(CompoundAssignmentsOp),
    /// `command`
    Command(Command),
    /// :comp:task(args)
    TaskCall(TaskCall),
}

impl Expression {
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::Accessor(n) => &n.uuid,
            Self::BinaryExp(n) => &n.uuid,
            Self::BinaryExpGroup(n) => &n.uuid,
            Self::BinaryExpSeq(n) => &n.uuid,
            Self::BinaryOp(n) => &n.uuid,
            Self::Call(n) => &n.uuid,
            Self::Command(n) => &n.uuid,
            Self::Comparison(n) => &n.uuid,
            Self::ComparisonGroup(n) => &n.uuid,
            Self::ComparisonOp(n) => &n.uuid,
            Self::ComparisonSeq(n) => &n.uuid,
            Self::CompoundAssignments(n) => &n.uuid,
            Self::CompoundAssignmentsOp(n) => &n.uuid,
            Self::FunctionCall(n) => &n.uuid,
            Self::LogicalOp(n) => &n.uuid,
            Self::Range(n) => &n.uuid,
            Self::TaskCall(n) => &n.uuid,
            Self::Variable(n) => &n.uuid,
        }
    }
}
