use crate::{
    error::LinkedErr,
    inf::{
        operator::{ExecuteContext, E},
        Value,
    },
};
use std::{future::Future, pin::Pin};

pub type ProcessingPinnedResult<'a> = Pin<Box<dyn Future<Output = ProcessingResult> + 'a + Send>>;
pub type ProcessingResult = Result<(), LinkedErr<E>>;

pub trait Processing {
    fn processing<'a>(
        &'a self,
        _results: &'a Value,
        _cx: ExecuteContext<'a>,
    ) -> ProcessingPinnedResult<'a> {
        Box::pin(async move { Ok(()) })
    }
}
