use crate::{
    elements::{conditions::Cmp, Comparing},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Comparing {}

impl TryExecute for Comparing {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let left = self.left.execute(cx.clone()).await?;
            let right = self.right.execute(cx.clone()).await?;
            Ok(match self.cmp {
                Cmp::LeftBig | Cmp::RightBig => {
                    let left = left.as_num().ok_or(E::FailToGetIntegerValue)?;
                    let right = right.as_num().ok_or(E::FailToGetIntegerValue)?;
                    Value::bool(
                        (matches!(self.cmp, Cmp::LeftBig) && left > right)
                            || matches!(self.cmp, Cmp::RightBig) && left < right,
                    )
                }
                Cmp::LeftBigInc | Cmp::RightBigInc => {
                    let left = left.as_num().ok_or(E::FailToGetIntegerValue)?;
                    let right = right.as_num().ok_or(E::FailToGetIntegerValue)?;
                    Value::bool(
                        (matches!(self.cmp, Cmp::LeftBigInc) && left >= right)
                            || matches!(self.cmp, Cmp::RightBigInc) && left <= right,
                    )
                }
                _ => {
                    // TODO: do not convert to string
                    let left = left.as_string().ok_or(E::FailToGetStringValue)?;
                    let right = right.as_string().ok_or(E::FailToGetStringValue)?;
                    Value::bool(
                        (matches!(self.cmp, Cmp::Equal) && left == right)
                            || (matches!(self.cmp, Cmp::NotEqual) && left != right),
                    )
                }
            })
        })
    }
}
