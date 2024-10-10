use crate::{
    elements::{typed::Types, VariableType},
    inf::{operator::E, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for VariableType {}

impl TryExecute for VariableType {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let value = if cx.args.len() != 1 {
                Err(E::InvalidNumberOfArgumentsForDeclaration)?
            } else {
                cx.args[0].to_owned()
            };
            Ok(match &self.var_type {
                Types::String => Value::String(value.as_string().ok_or(E::ParseStringError(
                    Types::String.to_string(),
                    "Value isn't String".to_string(),
                ))?),
                Types::Number => Value::isize(value.as_num().ok_or(E::ParseStringError(
                    Types::Number.to_string(),
                    "Value isn't number".to_string(),
                ))?),
                Types::Bool => Value::bool(value.as_bool().ok_or(E::ParseStringError(
                    Types::Bool.to_string(),
                    "Value isn't bool".to_string(),
                ))?),
            })
        })
    }
}
