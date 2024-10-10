use crate::{
    elements::{incrementer::Operator, Element, Incrementer, TokenGetter},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Incrementer {}

impl TryExecute for Incrementer {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let name = if let Element::VariableName(el, _) = &*self.variable {
                el.get_name()
            } else {
                return Err(E::NoVariableName.linked(&self.variable.token()));
            };
            let variable = self
                .variable
                .execute(cx.clone())
                .await?
                .as_num()
                .ok_or(E::ArithmeticWrongType.by(&*self.variable))?;
            let right = self
                .right
                .execute(cx.clone())
                .await?
                .as_num()
                .ok_or(E::ArithmeticWrongType.by(&*self.right))?;
            let changed = Value::isize(match self.operator {
                Operator::Inc => variable + right,
                Operator::Dec => variable - right,
            });
            cx.sc.set_var(&name, changed.duplicate()).await?;
            Ok(changed)
        })
    }
}
