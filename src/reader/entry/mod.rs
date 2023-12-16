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

pub fn read(filename: PathBuf) -> Result<Vec<Component>, E> {
    if !filename.exists() {
        Err(E::FileNotExists(filename.to_string_lossy().to_string()))?
    }
    let mut mapper = Mapper::new();
    let mut reader = Reader::new(fs::read_to_string(filename)?, &mut mapper, 0);
    let mut functions: Vec<Import> = vec![];
    while let Some(func) = Function::read(&mut reader)? {
        if let Some(fn_impl) = <Import as Implementation<Import, String>>::from(func)? {
            functions.push(fn_impl);
        } else {
            Err(E::NotAllowedFunction)?
        }
    }
    // Here should be import
    Ok(vec![])
}
