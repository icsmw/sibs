use crate::reader::{
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
    pub index: usize,
}

impl Reading<VariableDeclaration> for VariableDeclaration {
    fn read(reader: &mut crate::reader::Reader) -> Result<Option<VariableDeclaration>, E> {
        let from = reader.pos;
        if let Some(name) = VariableName::read(reader)? {
            if reader.move_to_char(&[chars::COLON])?.is_some() {
                if let Some(variable_type) = VariableType::read(reader)? {
                    Ok(Some(VariableDeclaration::typed(
                        name,
                        variable_type,
                        reader.get_index_until_current(from),
                    )))
                } else if let Some(values) = Values::read(reader)? {
                    Ok(Some(VariableDeclaration::values(
                        name,
                        values,
                        reader.get_index_until_current(from),
                    )))
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
    pub fn typed(name: VariableName, typed: VariableType, index: usize) -> Self {
        Self {
            name,
            declaration: Declaration::Typed(typed),
            index,
        }
    }
    pub fn values(name: VariableName, values: Values, index: usize) -> Self {
        Self {
            name,
            declaration: Declaration::Values(values),
            index,
        }
    }
}
