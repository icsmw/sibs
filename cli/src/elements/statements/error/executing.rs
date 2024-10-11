use crate::{
    elements::{Error, TokenGetter},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Error {}

impl TryExecute for Error {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            Ok(Value::Error(
                self.output
                    .execute(cx)
                    .await?
                    .as_string()
                    .ok_or(E::NotStringInError.linked(&self.output.token()))?,
            ))
        })
    }
}
