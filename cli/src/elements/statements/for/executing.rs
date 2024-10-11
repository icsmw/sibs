use crate::{
    elements::{Element, For, TokenGetter},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for For {}

impl TryExecute for For {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let Element::VariableName(variable, _) = self.index.as_ref() else {
                return Err(E::InvalidIndexVariableForStatement.linked(&self.target.token()));
            };
            let blk_token = if let Element::Block(el, _) = self.block.as_ref() {
                el.get_breaker()?
            } else {
                return Err(E::BlockElementExpected.linked(&self.block.token()));
            };
            let (loop_uuid, loop_token) = cx.sc.open_loop(blk_token).await?;
            match self.target.execute(cx.clone()).await? {
                Value::Range(v) => {
                    if v.len() != 2 {
                        return Err(E::InvalidRangeForStatement.linked(&self.target.token()));
                    }
                    let mut from = *v[0]
                        .get::<isize>()
                        .ok_or(E::InvalidRangeForStatement.linked(&self.target.token()))?;
                    let to = *v[1]
                        .get::<isize>()
                        .ok_or(E::InvalidRangeForStatement.linked(&self.target.token()))?;
                    let increase = from < to;
                    cx.sc
                        .set_var(&variable.get_name(), Value::isize(from))
                        .await?;
                    while from != to {
                        if loop_token.is_cancelled() {
                            break;
                        }
                        cx.sc
                            .set_var(&variable.get_name(), Value::isize(from))
                            .await?;
                        self.block.execute(cx.clone()).await?;
                        from += if increase { 1 } else { -1 };
                    }
                    cx.sc.close_loop(loop_uuid).await?;
                    Ok(Value::Empty(()))
                }
                Value::Vec(els) => {
                    for el in els.iter() {
                        if loop_token.is_cancelled() {
                            break;
                        }
                        cx.sc.set_var(&variable.get_name(), el.duplicate()).await?;
                        self.block.execute(cx.clone()).await?;
                    }
                    cx.sc.close_loop(loop_uuid).await?;
                    Ok(Value::Empty(()))
                }
                _ => Err(E::InvalidTargetForStatement.linked(&self.target.token())),
            }
        })
    }
}
