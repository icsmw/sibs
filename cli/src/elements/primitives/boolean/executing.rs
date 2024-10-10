use crate::{
    elements::Boolean,
    inf::{ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for Boolean {}

impl TryExecute for Boolean {
    fn try_execute<'a>(&'a self, _cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move { Ok(Value::bool(self.value)) })
    }
}
