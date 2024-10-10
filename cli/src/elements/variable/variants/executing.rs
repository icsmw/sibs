use crate::{
    elements::VariableVariants,
    inf::{operator::E, ExecuteContext, ExecutePinnedResult, Processing, TryExecute},
};

impl Processing for VariableVariants {}

impl TryExecute for VariableVariants {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let value = if cx.args.len() != 1 {
                Err(E::InvalidNumberOfArgumentsForDeclaration.by(self))?
            } else {
                cx.args[0].to_owned()
            };
            if self.values.contains(&value) {
                Ok(value)
            } else {
                Err(E::NotDeclaredValueAsArgument(
                    value.to_string(),
                    self.values
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" | "),
                )
                .by(self))
            }
        })
    }
}
