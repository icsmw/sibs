use crate::{
    elements::First,
    inf::{Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute},
};

impl Processing for First {}

impl TryExecute for First {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move { self.block.execute(cx).await })
    }
}
