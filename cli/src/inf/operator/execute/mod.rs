mod context;
pub use context::*;

use crate::{
    elements::{Metadata, TokenGetter},
    error::LinkedErr,
    inf::{
        operator::{Processing, TryExpectedValueType, E},
        PrevValue, Value,
    },
};
use std::{fmt::Debug, future::Future, pin::Pin};

pub type ExecutePinnedResult<'a> = Pin<Box<dyn Future<Output = ExecuteResult> + 'a + Send>>;
pub type ExecuteResult = Result<Value, LinkedErr<E>>;

pub trait TryExecute {
    #[allow(clippy::too_many_arguments)]
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a>;
}

pub trait Execute {
    #[allow(clippy::too_many_arguments)]
    fn execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a>
    where
        Self: TryExecute + Processing + TokenGetter + TryExpectedValueType + Debug + Sync,
    {
        Box::pin(async move {
            if cx.is_aborting() {
                cx.journal().warn("runner", "skipping, because aborting");
                return Err(E::Aborted.linked(&self.token()));
            }
            cx.atlas().set_map_position(self.token()).await?;
            let result = self.try_execute(cx.clone()).await;
            match result {
                Ok(value) => {
                    cx.atlas().add_footprint(self.token(), &value).await?;
                    let value = if let Some(ppm) = self.get_metadata()?.ppm.as_ref() {
                        ppm.execute(cx.clone().prev(&Some(PrevValue {
                            value,
                            token: self.token(),
                        })))
                        .await?
                    } else {
                        value
                    };
                    self.processing(&value, cx).await?;
                    Ok(value)
                }
                Err(err) => {
                    cx.atlas().report_err(&err.link_if(&self.token())).await?;
                    Err(err)
                }
            }
        })
    }
    fn get_metadata(&self) -> Result<&Metadata, LinkedErr<E>>
    where
        Self: TokenGetter + Debug + Sync,
    {
        Err(E::AttemptToGetMetadataOutOfElement(format!("{self:?}")).linked(&self.token()))
    }
}
