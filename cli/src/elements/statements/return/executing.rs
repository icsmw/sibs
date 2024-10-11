use crate::{
    elements::Return,
    inf::{Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for Return {}

impl TryExecute for Return {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            cx.sc
                .resolve(if let Some(el) = self.output.as_ref() {
                    el.execute(cx.clone()).await?
                } else {
                    Value::Empty(())
                })
                .await?;
            Ok(Value::Empty(()))
        })
    }
}
