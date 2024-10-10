use crate::{
    elements::{Element, VariableDeclaration},
    inf::{operator::E, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value},
};

impl Processing for VariableDeclaration {}

impl TryExecute for VariableDeclaration {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            cx.sc
                .set_var(
                    if let Element::VariableName(el, _) = self.variable.as_ref() {
                        &el.name
                    } else {
                        Err(E::FailToGetDeclaredVariable)?
                    },
                    self.get_val(cx.clone()).await?,
                )
                .await?;
            Ok(Value::empty())
        })
    }
}
