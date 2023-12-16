mod arguments;
mod block;
mod component;
mod embedded;
mod function;
mod group;
mod meta;
mod optional;
mod reference;
mod task;
mod value_strings;
mod values;
mod variable_assignation;
mod variable_comparing;
mod variable_declaration;
mod variable_name;
mod variable_type;

use crate::{
    error::E,
    functions::{reader::import::Import, Implementation},
    reader::{Mapper, Reader},
};
pub use arguments::{Argument, Arguments};
pub use block::Block;
pub use component::Component;
pub use embedded::{
    each::Each,
    first::First,
    If::{Cmp, If},
};
pub use function::Function;
pub use group::Group;
pub use meta::Meta;
pub use optional::Optional;
pub use reference::Reference;
use std::{fs, path::PathBuf};
pub use task::Task;
pub use value_strings::ValueString;
pub use values::Values;
pub use variable_assignation::VariableAssignation;
pub use variable_comparing::VariableComparing;
pub use variable_declaration::VariableDeclaration;
pub use variable_name::VariableName;
pub use variable_type::VariableType;

pub trait Reading<T> {
    fn read(reader: &mut Reader) -> Result<Option<T>, E>;
}
