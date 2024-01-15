use crate::{
    cli::error::E,
    inf::{any::DebugAny, reporter::Reporter},
    reader::entry::Component,
};

pub type Return = Option<Box<dyn DebugAny>>;
pub trait Runner {
    fn run(
        &self,
        components: &[Component],
        args: &[String],
        reporter: &mut Reporter,
    ) -> Result<Return, E>;
}
