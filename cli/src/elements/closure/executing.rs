use crate::{
    elements::Closure,
    inf::{ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for Closure {}

impl TryExecute for Closure {
    fn try_execute<'a>(&'a self, _cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move { Ok(Value::Closure(self.uuid)) })
    }
}
