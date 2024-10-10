use crate::{
    elements::Gatekeeper,
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Gatekeeper {}

impl TryExecute for Gatekeeper {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let condition = *self
                .function
                .execute(cx.clone())
                .await?
                .get::<bool>()
                .ok_or(E::FailToExtractConditionValue)?;
            Ok(Value::bool(condition))
        })
    }
}
