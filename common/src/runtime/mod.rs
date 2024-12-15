use crate::*;
use std::{future::Future, pin::Pin};

pub type RtPinnedResult<E> = Pin<Box<dyn Future<Output = RtResult<E>> + Send>>;
pub type RtResult<E> = Result<RtValue, E>;
