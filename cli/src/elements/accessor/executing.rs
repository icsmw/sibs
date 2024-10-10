use crate::{
    elements::Accessor,
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Accessor {}

impl TryExecute for Accessor {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let Some(prev_value) = cx.prev else {
                return Err(E::CallPPMWithoutPrevValue.linked(&self.token));
            };
            let n = self
                .index
                .execute(cx)
                .await?
                .as_num()
                .ok_or(E::FailToExtractAccessorIndex.by(&*self.index))?;
            if n < 0 {
                return Err(E::NegativeAccessorIndex(n).by(&*self.index));
            }
            let n = n as usize;
            Ok(match &prev_value.value {
                Value::String(v) => Value::String(
                    v.chars()
                        .nth(n)
                        .ok_or(E::OutOfBounds(v.chars().count(), n).linked(&self.token))?
                        .to_string(),
                ),
                Value::Vec(v) => v
                    .get(n)
                    .map(|v| v.duplicate())
                    .ok_or(E::OutOfBounds(v.len(), n).linked(&self.token))?,
                _ => {
                    Err(E::AccessByIndexNotSupported(prev_value.value.to_string())
                        .linked(&self.token))?
                }
            })
        })
    }
}
