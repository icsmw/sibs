use crate::{
    cli::error::E,
    inf::{any::DebugAny, context::Context},
    reader::entry::Component,
};

pub type Return = Option<Box<dyn DebugAny>>;
pub trait Runner {
    fn run(
        &self,
        components: &[Component],
        args: &[String],
        context: &mut Context,
    ) -> Result<Return, E>;
}
