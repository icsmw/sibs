use crate::{
    elements::function::{FuncArg, Function},
    inf::{ExecuteContext, ExecutePinnedResult, Processing, TryExecute},
};

impl Processing for Function {}

impl TryExecute for Function {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let args: Vec<FuncArg> = self.get_processed_args(cx.clone()).await?;
            Ok(cx
                .cx
                .execute(
                    &self.name,
                    args,
                    self.args_token,
                    cx.prev.as_ref().map(|v| v.value.clone()).clone(),
                    cx.sc,
                )
                .await?)
        })
    }
}
