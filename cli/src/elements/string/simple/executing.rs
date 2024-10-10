use crate::{
    elements::SimpleString,
    inf::{ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for SimpleString {}

impl TryExecute for SimpleString {
    fn try_execute<'a>(&'a self, _cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move { Ok(Value::String(self.value.to_string())) })
    }
}
