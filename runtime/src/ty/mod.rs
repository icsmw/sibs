use crate::*;
use std::{future::Future, pin::Pin};

pub type RtPinnedResult<'a, E> = Pin<Box<dyn Future<Output = RtResult<E>> + 'a + Send>>;
pub type RtResult<E> = Result<RtValue, E>;
