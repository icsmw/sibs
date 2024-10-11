use crate::{
    elements::IfCondition,
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for IfCondition {}

impl TryExecute for IfCondition {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            Ok(Value::bool(
                *self
                    .subsequence
                    .execute(cx)
                    .await?
                    .get::<bool>()
                    .ok_or(E::NoBoolValueFromSubsequence)?,
            ))
        })
    }
}
