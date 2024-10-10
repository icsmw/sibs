use crate::{
    elements::{Range, TokenGetter},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Range {}

impl TryExecute for Range {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let from = self
                .from
                .execute(cx.clone())
                .await?
                .as_num()
                .ok_or(E::ExpectedNumericValue.linked(&self.from.token()))?;
            let to = self
                .to
                .execute(cx)
                .await?
                .as_num()
                .ok_or(E::ExpectedNumericValue.linked(&self.to.token()))?;
            Ok(Value::Range(vec![Value::isize(from), Value::isize(to)]))
        })
    }
}
