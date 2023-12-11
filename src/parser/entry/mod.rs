mod arguments;
mod block;
mod component;
mod condition;
mod function;
mod group;
mod optional;
mod task;
mod value_strings;
mod values;
mod variable_assignation;
mod variable_declaration;
mod variable_name;
mod variable_type;

use crate::parser::{chars, Reader, E};
pub use arguments::Arguments;
pub use block::Block;
pub use component::Component;
pub use condition::Condition;
pub use function::Function;
pub use group::Group;
pub use optional::Optional;
pub use task::Task;
pub use value_strings::ValueString;
pub use values::Values;
pub use variable_assignation::VariableAssignation;
pub use variable_declaration::VariableDeclaration;
pub use variable_name::VariableName;
pub use variable_type::VariableType;

pub trait Reading<T> {
    fn read(reader: &mut Reader) -> Result<Option<T>, E>;
}
#[derive(Debug)]
pub enum Entry {
    // /// [...]
    // SBracketBlock(Data),
    // /// (...)
    // BracketBlock(Data),
    // /// @...
    Function(Function),
    Arguments(Arguments),
    // /// $...
    VariableName(VariableName),
    VariableType(VariableType),
    Values(Values),
    VariableDeclaration(VariableDeclaration),
    Group(Group),
    Component(Component),
    // /// ///...
    // Meta(Data),
    // /// :...:...
    // Reference(Data),
    // /// ...
    // Unknown(Data),
}

impl Entry {
    pub fn parse(reader: &mut Reader) -> Result<Vec<Self>, E> {
        let mut entities: Vec<Self> = vec![];
        loop {
            if let Some(entity) = Self::function(reader)? {
                entities.push(entity);
            } else if let Some(entity) = Self::component(reader)? {
                entities.push(entity);
            } else {
                break;
            }
        }
        Ok(entities)
    }

    pub fn component(reader: &mut Reader) -> Result<Option<Self>, E> {
        if reader.move_to_char(chars::POUND_SIGN)? {
            if let Some(Self::Group(group)) = Self::group(reader)? {
                Ok(Some(Self::Component(Component::new(group, reader)?)))
            } else {
                Err(E::NoGroup)
            }
        } else {
            Ok(None)
        }
    }
    pub fn function(reader: &mut Reader) -> Result<Option<Self>, E> {
        Function::read(reader)?.map_or(Ok(None), |v| Ok(Some(Self::Function(v))))
    }
    pub fn arguments(reader: &mut Reader) -> Result<Option<Self>, E> {
        Arguments::read(reader)?.map_or(Ok(None), |v| Ok(Some(Self::Arguments(v))))
    }
    pub fn group(reader: &mut Reader) -> Result<Option<Self>, E> {
        Group::read(reader)?.map_or(Ok(None), |v| Ok(Some(Self::Group(v))))
    }
    pub fn variable_name(reader: &mut Reader) -> Result<Option<Self>, E> {
        VariableName::read(reader)?.map_or(Ok(None), |v| Ok(Some(Self::VariableName(v))))
    }
    pub fn variable_declaration(reader: &mut Reader) -> Result<Option<Self>, E> {
        VariableDeclaration::read(reader)?
            .map_or(Ok(None), |v| Ok(Some(Self::VariableDeclaration(v))))
    }
    pub fn variable_type(reader: &mut Reader) -> Result<Option<Self>, E> {
        VariableType::read(reader)?.map_or(Ok(None), |v| Ok(Some(Self::VariableType(v))))
    }
    pub fn values(reader: &mut Reader) -> Result<Option<Self>, E> {
        Values::read(reader)?.map_or(Ok(None), |v| Ok(Some(Self::Values(v))))
    }
}
