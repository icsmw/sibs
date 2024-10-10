use crate::{
    elements::Combination,
    inf::{ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for Combination {}

impl TryExecute for Combination {
    fn try_execute<'a>(&'a self, _cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move { Ok(Value::Cmb(self.cmb.clone())) })
    }
}
