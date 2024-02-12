use crate::{
    entry::{VariableName, VariableType, Variants},
    inf::{any::AnyValue, context::Context, operator, term},
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Declaration {
    Typed(VariableType),
    Variants(Variants),
}

impl Declaration {
    pub fn token(&self) -> usize {
        match &self {
            Declaration::Typed(v) => v.token,
            Declaration::Variants(v) => v.token,
        }
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Declaration::Typed(v) => v.to_string(),
                Declaration::Variants(v) => v.to_string(),
            }
        )
    }
}

#[derive(Debug, Clone)]
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
                    Declaration::Variants(values) => values.parse(value),
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
        let close = reader.open_token();
        if let Some(name) = VariableName::read(reader)? {
            if reader.move_to().char(&[&chars::COLON]).is_some() {
                let declaration = if let Some(variable_type) = VariableType::read(reader)? {
                    Some(VariableDeclaration::typed(
                        name,
                        variable_type,
                        close(reader),
                    ))
                } else if let Some(values) = Variants::read(reader)? {
                    Some(VariableDeclaration::values(name, values, close(reader)))
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
    pub fn values(name: VariableName, values: Variants, token: usize) -> Self {
        Self {
            name,
            declaration: Declaration::Variants(values),
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
                Declaration::Variants(v) => v.to_string(),
            }
        )
    }
}

impl term::Display for VariableDeclaration {
    fn to_string(&self) -> String {
        match &self.declaration {
            Declaration::Typed(v) => term::Display::to_string(v),
            Declaration::Variants(v) => term::Display::to_string(v),
        }
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        entry::{
            variable_declaration::{Declaration, VariableDeclaration},
            variable_name::VariableName,
            variable_type::VariableType,
            variants::Variants,
        },
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Declaration {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Variants::arbitrary_with(scope.clone()).prop_map(Declaration::Variants),
                VariableType::arbitrary_with(scope.clone()).prop_map(Declaration::Typed),
            ]
            .boxed()
        }
    }
    impl Arbitrary for VariableDeclaration {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            (
                Declaration::arbitrary_with(scope.clone()).prop_map(|v| v),
                VariableName::arbitrary().prop_map(|v| v),
            )
                .prop_map(move |(declaration, name)| {
                    scope.write().unwrap().add_declaration(name.name.clone());
                    VariableDeclaration {
                        declaration,
                        name,
                        token: 0,
                    }
                })
                .boxed()
        }
    }
}
