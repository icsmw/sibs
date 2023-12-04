mod arguments;
mod component;
mod function;
mod group;
mod values;
mod variable_declaration;
mod variable_name;
mod variable_type;

use crate::parser::{chars, Reader, E};
pub use arguments::Arguments;
pub use component::Component;
pub use function::Function;
pub use group::Group;
pub use values::Values;
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
                println!("{entity:?}");
                entities.push(entity);
            } else if let Some(entity) = Self::component(reader)? {
                println!("{entity:?}");
                entities.push(entity);
            } else {
                println!("{}", reader.rest());
                println!(">>>>>>>>>>>>>>>>>>>>> break");
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
        if reader.move_to_char(chars::DOLLAR)? {
            if let Some((word, _, uuid)) = reader.read_letters(&[chars::COLON], false)? {
                Ok(Some(Self::VariableName(VariableName::new(word))))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    pub fn variable_declaration(reader: &mut Reader) -> Result<Option<Self>, E> {
        if reader.move_to_char(chars::COLON)? {
            if let Some(Self::VariableType(variable_type)) = Self::variable_type(reader)? {
                Ok(Some(Self::VariableDeclaration(VariableDeclaration::typed(
                    variable_type,
                ))))
            } else if let Some(Self::Values(values)) = Self::values(reader)? {
                Ok(Some(Self::VariableDeclaration(
                    VariableDeclaration::values(values),
                )))
            } else {
                Err(E::NoTypeDeclaration)
            }
        } else {
            Ok(None)
        }
    }

    pub fn variable_type(reader: &mut Reader) -> Result<Option<Self>, E> {
        if reader.move_to_char(chars::TYPE_OPEN)? {
            if let Some((word, _)) = reader.read_word(&[chars::TYPE_CLOSE], true)? {
                Ok(Some(Self::VariableType(VariableType::new(word)?)))
            } else {
                Err(E::NotClosedTypeDeclaration)
            }
        } else {
            Ok(None)
        }
    }

    pub fn values(reader: &mut Reader) -> Result<Option<Self>, E> {
        if let Some((variants, _stopped_on, _uuid)) =
            reader.read_until(&[chars::SEMICOLON, chars::CLOSE_BRACKET], true)?
        {
            Ok(Some(Self::Values(Values::new(variants)?)))
        } else {
            Err(E::NoTypeDeclaration)
        }
    }
}
