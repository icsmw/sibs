use crate::{
    elements::{conditions::Cmb, Subsequence},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Subsequence {}

impl TryExecute for Subsequence {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let mut last_value = true;
            for el in self.subsequence.iter() {
                let value = el.execute(cx.clone()).await?;
                if let Some(cmb) = value.get::<Cmb>() {
                    match cmb {
                        Cmb::And => {
                            if !last_value {
                                return Ok(Value::bool(false));
                            }
                        }
                        Cmb::Or => {
                            if last_value {
                                return Ok(Value::bool(true));
                            }
                        }
                    }
                } else if let Some(value) = value.as_bool() {
                    last_value = value;
                } else {
                    Err(E::FailToParseValueOfSubsequenceElement)?;
                }
            }
            Ok(Value::bool(last_value))
        })
    }
}
