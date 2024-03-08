use crate::{
    entry::{VariableName, VariableType, VariableVariants},
    error::LinkedErr,
    inf::{any::AnyValue, context::Context, operator, term},
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Declaration {
    Typed(VariableType),
    VariableVariants(VariableVariants),
}

impl Declaration {
    pub fn token(&self) -> usize {
        match &self {
            Declaration::Typed(v) => v.token,
            Declaration::VariableVariants(v) => v.token,
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
                Declaration::VariableVariants(v) => v.to_string(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub variable: VariableName,
    pub declaration: Declaration,
    pub token: usize,
}

impl VariableDeclaration {
    pub async fn declare<'a>(&self, value: String, cx: &'a mut Context) -> Result<(), operator::E> {
        cx.set_var(
            self.variable.name.to_owned(),
            AnyValue::new(
                match &self.declaration {
                    Declaration::Typed(typed) => typed.parse(value),
                    Declaration::VariableVariants(values) => values.parse(value),
                }
                .ok_or(operator::E::NoValueToDeclareTaskArgument)?,
            ),
        );
        Ok(())
    }
}

impl Reading<VariableDeclaration> for VariableDeclaration {
    fn read(reader: &mut Reader) -> Result<Option<VariableDeclaration>, LinkedErr<E>> {
        let close = reader.open_token();
        if let Some(variable) = VariableName::read(reader)? {
            if reader.move_to().char(&[&chars::COLON]).is_some() {
                let declaration = if let Some(variable_type) = VariableType::read(reader)? {
                    Some(VariableDeclaration::typed(
                        variable,
                        variable_type,
                        close(reader),
                    ))
                } else if let Some(values) = VariableVariants::read(reader)? {
                    Some(VariableDeclaration::values(variable, values, close(reader)))
                } else {
                    return Err(E::NoTypeDeclaration.by_reader(reader));
                };
                reader.trim();
                if matches!(reader.next().char(), Some(chars::SEMICOLON)) {
                    reader.move_to().next();
                }
                Ok(declaration)
            } else {
                Err(E::NoTypeDeclaration.linked(&variable.token))
            }
        } else {
            Ok(None)
        }
    }
}

impl VariableDeclaration {
    pub fn typed(variable: VariableName, typed: VariableType, token: usize) -> Self {
        Self {
            variable,
            declaration: Declaration::Typed(typed),
            token,
        }
    }
    pub fn values(variable: VariableName, values: VariableVariants, token: usize) -> Self {
        Self {
            variable,
            declaration: Declaration::VariableVariants(values),
            token,
        }
    }
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variable,
            match &self.declaration {
                Declaration::Typed(v) => v.to_string(),
                Declaration::VariableVariants(v) => v.to_string(),
            }
        )
    }
}

impl term::Display for VariableDeclaration {
    fn to_string(&self) -> String {
        match &self.declaration {
            Declaration::Typed(v) => term::Display::to_string(v),
            Declaration::VariableVariants(v) => term::Display::to_string(v),
        }
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        entry::variable::{
            Declaration, VariableDeclaration, VariableName, VariableType, VariableVariants,
        },
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Declaration {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                VariableVariants::arbitrary_with(scope.clone())
                    .prop_map(Declaration::VariableVariants),
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
                .prop_map(move |(declaration, variable)| {
                    scope
                        .write()
                        .unwrap()
                        .add_declaration(variable.name.clone());
                    VariableDeclaration {
                        declaration,
                        variable,
                        token: 0,
                    }
                })
                .boxed()
        }
    }
}
