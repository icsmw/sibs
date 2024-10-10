use crate::{
    elements::Integer,
    inf::{ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for Integer {}

impl TryExecute for Integer {
    fn try_execute<'a>(&'a self, _cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move { Ok(Value::isize(self.value)) })
    }
}
