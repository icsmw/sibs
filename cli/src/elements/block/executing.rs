use crate::{
    elements::{Block, ElementRef},
    inf::{Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for Block {}

impl TryExecute for Block {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let mut output = Value::empty();
            for element in self.elements.iter() {
                if let Some(breaker) = self.breaker.as_ref() {
                    if breaker.is_cancelled() {
                        return Ok(output);
                    }
                }
                if let Some(retreat) = cx.sc.get_retreat().await? {
                    return Ok(retreat);
                }
                output = element.execute(cx.clone()).await?;
                if let (Some(ElementRef::First), false) = (self.owner.as_ref(), output.is_empty()) {
                    return Ok(output);
                }
            }
            Ok(output)
        })
    }
}
