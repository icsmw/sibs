mod conflict;
mod interest;
mod read;

mod accessor;
mod binary_exp;
mod binary_op;
mod call;
mod command;
mod comparison;
mod comparison_group;
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
pub use comparison_group::*;
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
    /// .as_str(), .is_err(), call of function after expression
    Call(Call),
    /// [1], [n], [get_index()], access to indexed value
    Accessor(Accessor),
    /// Do I need this still?
    Condition(Condition),
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
    /// 1 + 2, 1 / 2, (x + 2) / 3, etc.
    BinaryExp(BinaryExp),
    /// +, -, *, /
    BinaryOp(BinaryOp),
    /// func(), get_os(), etc.
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
