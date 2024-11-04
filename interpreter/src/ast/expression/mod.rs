mod conflict;
mod interest;
mod read;

mod accessor;
mod binary_exp;
mod call;
mod command;
mod comparing;
mod comparing_seq;
mod comparison_op;
mod condition;
mod function_call;
mod incrementer;
mod logical_op;
mod range;
mod task_call;
mod variable;

pub use accessor::*;
pub use binary_exp::*;
pub use call::*;
pub use command::*;
pub use comparing::*;
pub use comparing_seq::*;
pub use comparison_op::*;
pub use condition::*;
pub use function_call::*;
pub use incrementer::*;
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
    Comparing(Comparing),
    /// &&, ||
    LogicalOp(LogicalOp),
    /// ==, >=, <=, !=
    ComparisonOp(ComparisonOp),
    ComparingSeq(ComparingSeq),
    Range(Range),
    Variable(Variable),
    BinaryExp(BinaryExp),
    FunctionCall(FunctionCall),
    Incrementer(Incrementer),
    Command(Command),
    TaskCall(TaskCall),
}
