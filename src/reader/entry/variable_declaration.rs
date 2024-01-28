use crate::{
    inf::{any::AnyValue, context::Context, operator, term},
    reader::{
        chars,
        entry::{Reader, Reading, Values, VariableName, VariableType},
        E,
    },
};
use std::fmt;

#[derive(Debug)]
pub enum Declaration {
    Typed(VariableType),
    Values(Values),
}
#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: VariableName,
    pub declaration: Declaration,
    pub token: usize,
}

impl VariableDeclaration {
    pub async fn declare<'a>(&self, value: String, cx: &'a mut Context) -> Result<(), operator::E> {
        cx.set_var(
            self.name.name.to_owned(),
            AnyValue::new(
                match &self.declaration {
                    Declaration::Typed(typed) => typed.parse(value),
                    Declaration::Values(values) => values.parse(value),
                }
                .ok_or(operator::E::NoValueToDeclareTaskArgument)?,
            ),
        )
        .await;
        Ok(())
    }
}

impl Reading<VariableDeclaration> for VariableDeclaration {
    fn read(reader: &mut Reader) -> Result<Option<VariableDeclaration>, E> {
        if let Some(name) = VariableName::read(reader)? {
            if reader.move_to().char(&[&chars::COLON]).is_some() {
                let declaration = if let Some(variable_type) = VariableType::read(reader)? {
                    Some(VariableDeclaration::typed(
                        name,
                        variable_type,
                        reader.token()?.id,
                    ))
                } else if let Some(values) = Values::read(reader)? {
                    Some(VariableDeclaration::values(
                        name,
                        values,
                        reader.token()?.id,
                    ))
                } else {
                    return Err(E::NoTypeDeclaration);
                };
                reader.trim();
                if matches!(reader.next().char(), Some(chars::SEMICOLON)) {
                    reader.move_to().next();
                }
                Ok(declaration)
            } else {
                Err(E::NoTypeDeclaration)
            }
        } else {
            Ok(None)
        }
    }
}

impl VariableDeclaration {
    pub fn typed(name: VariableName, typed: VariableType, token: usize) -> Self {
        Self {
            name,
            declaration: Declaration::Typed(typed),
            token,
        }
    }
    pub fn values(name: VariableName, values: Values, token: usize) -> Self {
        Self {
            name,
            declaration: Declaration::Values(values),
            token,
        }
    }
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name,
            match &self.declaration {
                Declaration::Typed(v) => v.to_string(),
                Declaration::Values(v) => v.to_string(),
            }
        )
    }
}

impl term::Display for VariableDeclaration {
    fn to_string(&self) -> String {
        match &self.declaration {
            Declaration::Typed(v) => term::Display::to_string(v),
            Declaration::Values(v) => term::Display::to_string(v),
        }
    }
}
