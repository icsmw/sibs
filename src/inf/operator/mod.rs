mod error;
use crate::{
    inf::{any::AnyValue, context::Context},
    reader::entry::Component,
};
pub use error::E;

pub trait Operator {
    async fn process(
        &self,
        components: &[Component],
        args: &[String],
        cx: &mut Context,
    ) -> Result<Option<AnyValue>, E> {
        Err(E::NotSupported)
    }
}
