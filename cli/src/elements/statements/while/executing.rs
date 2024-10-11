use crate::{
    elements::{Element, TokenGetter, While},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for While {}

impl TryExecute for While {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let blk_token = if let Element::Block(el, _) = self.block.as_ref() {
                el.get_breaker()?
            } else {
                return Err(E::BlockElementExpected.linked(&self.block.token()));
            };
            let (loop_uuid, loop_token) = cx.sc.open_loop(blk_token).await?;
            let mut n = u64::MIN;
            while n < u64::MAX {
                if loop_token.is_cancelled() {
                    break;
                }
                if !self
                    .condition
                    .execute(cx.clone())
                    .await?
                    .as_bool()
                    .ok_or(E::ConditionReturnsNotBool.linked(&self.condition.token()))?
                {
                    break;
                }
                if n == u64::MAX - 1 {
                    cx.sc.close_loop(loop_uuid).await?;
                    return Err(E::MaxIterations.linked(&self.token));
                }
                self.block.execute(cx.clone()).await?;
                n += 1;
            }
            cx.sc.close_loop(loop_uuid).await?;
            Ok(Value::Empty(()))
        })
    }
}
