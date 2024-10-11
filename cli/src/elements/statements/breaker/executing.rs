use crate::{
    elements::Breaker,
    inf::{ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for Breaker {}

impl TryExecute for Breaker {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            cx.sc.break_loop().await?;
            Ok(Value::empty())
        })
    }
}
