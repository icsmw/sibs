use crate::{
    elements::Call,
    inf::{operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute},
};

impl TryExecute for Call {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let Some(prev_value) = cx.prev else {
                return Err(E::CallPPMWithoutPrevValue.linked(&self.token));
            };
            self.func
                .execute(cx.clone().prev(&Some(prev_value.clone())))
                .await
        })
    }
}

impl Processing for Call {}
