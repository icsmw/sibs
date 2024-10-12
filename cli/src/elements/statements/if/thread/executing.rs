use crate::{
    elements::IfThread,
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl TryExecute for IfThread {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            match self {
                Self::If(subsequence, block) => {
                    if *subsequence
                        .execute(cx.clone())
                        .await?
                        .get::<bool>()
                        .ok_or(E::NoBoolResultFromProviso)?
                    {
                        block.execute(cx).await
                    } else {
                        Ok(Value::empty())
                    }
                }
                Self::Else(block) => block.execute(cx).await,
            }
        })
    }
}

impl Processing for IfThread {}
