use crate::{
    elements::{Each, Element, TokenGetter},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Each {}

impl TryExecute for Each {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let inputs = self
                .input
                .execute(cx.clone())
                .await?
                .as_strings()
                .ok_or(E::FailConvertInputIntoStringsForEach)?;
            let mut output: Value = Value::empty();
            let blk_token = if let Element::Block(el, _) = self.block.as_ref() {
                el.get_breaker()?
            } else {
                return Err(E::BlockElementExpected.linked(&self.block.token()));
            };
            let (loop_uuid, loop_token) = cx.sc.open_loop(blk_token).await?;
            let Element::VariableName(variable, _) = self.variable.as_ref() else {
                return Err(E::NoVariableName.by(self.variable.as_ref()));
            };
            for iteration in inputs.iter() {
                if loop_token.is_cancelled() {
                    break;
                }
                cx.sc
                    .set_var(&variable.name, Value::String(iteration.to_string()))
                    .await?;
                output = self.block.execute(cx.clone()).await?;
            }
            cx.sc.close_loop(loop_uuid).await?;
            Ok(output)
        })
    }
}
