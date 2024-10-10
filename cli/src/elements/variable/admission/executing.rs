use crate::{
    elements::VariableName,
    inf::{operator::E, ExecuteContext, ExecutePinnedResult, Processing, TryExecute},
};

impl Processing for VariableName {}

impl TryExecute for VariableName {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            Ok(cx
                .sc
                .get_var(&self.name)
                .await?
                .ok_or(E::VariableIsNotAssigned(self.name.to_owned()))?
                .duplicate())
        })
    }
}
