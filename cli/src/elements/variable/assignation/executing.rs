use crate::{
    elements::{Element, VariableAssignation},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for VariableAssignation {}

impl TryExecute for VariableAssignation {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let Element::VariableName(variable, _) = self.variable.as_ref() else {
                return Err(E::NoVariableName.by(self.variable.as_ref()));
            };
            let value = self
                .assignation
                .execute(cx.clone())
                .await?
                .not_empty_or(E::NoValueToAssign(variable.name.clone()))?;
            if self.global {
                cx.sc.set_global_var(&variable.name, value).await?;
            } else {
                cx.sc.set_var(&variable.name, value).await?;
            }
            Ok(Value::empty())
        })
    }
}
