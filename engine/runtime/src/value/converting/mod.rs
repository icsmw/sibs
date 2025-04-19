mod primitive;
mod vec;

use crate::*;

pub trait TryToRs<T> {
    fn try_to_rs(self) -> Result<T, E>;
}

pub trait TryToRtValue {
    fn try_to_rtv(self) -> Result<RtValue, E>;
}
