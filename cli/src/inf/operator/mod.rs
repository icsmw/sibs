mod error;
mod token;

use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{AnyValue, Context, Scope},
};
pub use error::E;
use std::{future::Future, pin::Pin};
pub use token::*;

pub type OperatorPinnedResult<'a> = Pin<Box<dyn Future<Output = OperatorResult> + 'a + Send>>;
pub type OperatorResult = Result<Option<AnyValue>, LinkedErr<E>>;

pub trait Operator {
    fn token(&self) -> usize;
    fn execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: Context,
        sc: Scope,
        token: OperatorToken,
    ) -> OperatorPinnedResult
    where
        Self: Sync,
    {
        Box::pin(async move {
            cx.atlas.set_map_position(self.token()).await?;
            let confirmation = token.get_confirmation();
            let result = self
                .perform(owner, components, args, cx.clone(), sc, token)
                .await;
            match result.as_ref() {
                Ok(value) => {
                    cx.atlas.add_footprint(self.token(), value).await?;
                }
                Err(err) => {
                    cx.atlas.report_err(&err.link_if(&self.token())).await?;
                }
            }
            confirmation();
            println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> EXECUTING DONE");
            result
        })
    }
    fn perform<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        _args: &'a [String],
        _cx: Context,
        _sc: Scope,
        _token: OperatorToken,
    ) -> OperatorPinnedResult {
        Box::pin(async { Err(E::NotSupported.unlinked()) })
    }
}
