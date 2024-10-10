use crate::{
    elements::{compute::Operator, Compute},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Compute {}

impl TryExecute for Compute {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let left = self
                .left
                .execute(cx.clone())
                .await?
                .as_num()
                .ok_or(E::ArithmeticWrongType.by(&*self.left))?;
            let right = self
                .right
                .execute(cx.clone())
                .await?
                .as_num()
                .ok_or(E::ArithmeticWrongType.by(&*self.right))?;
            Ok(match self.operator {
                Operator::Inc => Value::isize(left + right),
                Operator::Dec => Value::isize(left - right),
                Operator::Div => Value::isize(left / right),
                Operator::Mlt => Value::isize(left * right),
            })
        })
    }
}
