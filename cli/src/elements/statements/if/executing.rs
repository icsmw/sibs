use crate::{
    elements::If,
    inf::{ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for If {}

impl TryExecute for If {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            for thread in self.threads.iter() {
                let output = thread.try_execute(cx.clone()).await?;
                if !output.is_empty() {
                    return Ok(output);
                }
            }
            Ok(Value::empty())
        })
    }
}
