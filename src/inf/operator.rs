use crate::{
    cli::error::E,
    inf::{any::AnyValue, context::Context},
    reader::entry::Component,
};

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
