mod conflict;
mod interest;
mod read;

mod accessor;
mod binary_exp;
mod call;
mod command;
mod comparing;
mod comparing_seq;
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
pub use condition::*;
pub use function_call::*;
pub use incrementer::*;
pub use logical_op::*;
pub use range::*;
pub use task_call::*;
pub use variable::*;

use std::fmt;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone")]
#[derive(Debug, Clone)]
pub enum Expression {
    Call(Call),
    Accessor(Accessor),
    Condition(Condition),
    Comparing(Comparing),
    LogicalOp(LogicalOp),
    ComparingSeq(ComparingSeq),
    Range(Range),
    Variable(Variable),
    BinaryExp(BinaryExp),
    FunctionCall(FunctionCall),
    Incrementer(Incrementer),
    Command(Command),
    TaskCall(TaskCall),
}

impl fmt::Display for ExpressionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Call => "Expression::Call",
                Self::Accessor => "Expression::Accessor",
                Self::Variable => "Expression::Variable",
                Self::Condition => "Expression::Condition",
                Self::Comparing => "Expression::Comparing",
                Self::LogicalOp => "Expression::LogicalOp",
                Self::ComparingSeq => "Expression::ComparingSeq",
                Self::Range => "Expression::Range",
                Self::BinaryExp => "Expression::BinaryExp",
                Self::FunctionCall => "Expression::FunctionCall",
                Self::Incrementer => "Expression::Incrementer",
                Self::Command => "Expression::Command",
                Self::TaskCall => "Expression::TaskCall",
            }
        )
    }
}
