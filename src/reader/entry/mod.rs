mod arguments;
mod block;
mod command;
mod component;
mod embedded;
mod function;
mod meta;
mod optional;
mod pattern_string;
mod reference;
mod simple_string;
mod task;
mod values;
mod variable_assignation;
mod variable_comparing;
mod variable_declaration;
mod variable_name;
mod variable_type;
mod variants;

use crate::reader::{error::E, Reader};
pub use arguments::{Argument, Arguments};
pub use block::Block;
pub use command::Command;
pub use component::Component;
pub use embedded::{
    each::Each,
    first::First,
    If::{Cmp, If},
};
pub use function::Function;
pub use meta::Meta;
pub use optional::Optional;
pub use pattern_string::PatternString;
pub use reference::Reference;
pub use simple_string::SimpleString;
pub use task::Task;
pub use values::Values;
pub use variable_assignation::VariableAssignation;
pub use variable_comparing::VariableComparing;
pub use variable_declaration::VariableDeclaration;
pub use variable_name::VariableName;
pub use variable_type::VariableType;
pub use variants::Variants;

pub trait Reading<T> {
    fn read(reader: &mut Reader) -> Result<Option<T>, E>;
}
