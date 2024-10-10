use crate::{
    elements::Values,
    inf::{Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for Values {}

impl TryExecute for Values {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let mut values: Vec<Value> = Vec::new();
            for el in self.elements.iter() {
                values.push(el.execute(cx.clone()).await?);
            }
            Ok(Value::Vec(values))
        })
    }
}
