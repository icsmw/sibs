use crate::parser::{
    chars,
    entry::{Reading, Values, VariableName, VariableType},
    E,
};

#[derive(Debug)]
pub enum Declaration {
    Typed(VariableType),
    Values(Values),
}
#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: VariableName,
    pub declaration: Declaration,
}

impl Reading<VariableDeclaration> for VariableDeclaration {
    fn read(reader: &mut crate::parser::Reader) -> Result<Option<VariableDeclaration>, E> {
        if let Some(name) = VariableName::read(reader)? {
            if reader.move_to_char(&[chars::COLON])?.is_some() {
                if let Some(variable_type) = VariableType::read(reader)? {
                    Ok(Some(VariableDeclaration::typed(name, variable_type)))
                } else if let Some(values) = Values::read(reader)? {
                    Ok(Some(VariableDeclaration::values(name, values)))
                } else {
                    Err(E::NoTypeDeclaration)
                }
            } else {
                Err(E::NoTypeDeclaration)
            }
        } else {
            Ok(None)
        }
    }
}

impl VariableDeclaration {
    pub fn typed(name: VariableName, typed: VariableType) -> Self {
        Self {
            name,
            declaration: Declaration::Typed(typed),
        }
    }
    pub fn values(name: VariableName, values: Values) -> Self {
        Self {
            name,
            declaration: Declaration::Values(values),
        }
    }
}
