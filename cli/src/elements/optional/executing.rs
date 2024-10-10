use crate::{
    elements::Optional,
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Optional {}

impl TryExecute for Optional {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let condition = *self
                .condition
                .execute(cx.clone())
                .await?
                .get::<bool>()
                .ok_or(E::FailToExtractConditionValue)?;
            if !condition {
                Ok(Value::empty())
            } else {
                self.action.execute(cx).await
            }
        })
    }
}
