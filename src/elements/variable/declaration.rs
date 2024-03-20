use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{any::AnyValue, context::Context, operator, term},
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub variable: Box<Element>,
    pub declaration: Box<Element>,
    pub token: usize,
}

impl VariableDeclaration {
    pub async fn declare<'a>(&self, value: String, cx: &'a mut Context) -> Result<(), operator::E> {
        cx.set_var(
            if let Element::VariableName(el, _) = self.variable.as_ref() {
                el.name.to_owned()
            } else {
                Err(operator::E::FailToGetDeclaredVariable)?
            },
            AnyValue::new(
                if let Element::VariableType(el, _) = self.declaration.as_ref() {
                    el.parse(value)
                } else if let Element::VariableVariants(el, _) = self.declaration.as_ref() {
                    el.parse(value)
                } else {
                    Err(operator::E::FailToExtractValue)?
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
        if let Some(variable) = Element::include(reader, &[ElTarget::VariableName])? {
            if reader.move_to().char(&[&chars::COLON]).is_some() {
                if let Some(declaration) = Element::include(
                    reader,
                    &[ElTarget::VariableType, ElTarget::VariableVariants],
                )? {
                    Ok(Some(VariableDeclaration {
                        variable: Box::new(variable),
                        declaration: Box::new(declaration),
                        token: close(reader),
                    }))
                } else {
                    Err(E::NoTypeDeclaration.by_reader(reader))
                }
            } else {
                Err(E::NoTypeDeclaration.by_reader(reader))
            }
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.variable, self.declaration)
    }
}

impl term::Display for VariableDeclaration {
    fn to_string(&self) -> String {
        // term::Display::to_string(self.declaration)
        String::new()
    }
}

#[cfg(test)]
mod proptest {
    use crate::elements::{ElTarget, Element, VariableDeclaration};
    use proptest::prelude::*;

    impl Arbitrary for VariableDeclaration {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((
                    vec![ElTarget::VariableType, ElTarget::VariableVariants],
                    deep,
                )),
                Element::arbitrary_with((vec![ElTarget::VariableName], deep)),
            )
                .prop_map(move |(declaration, variable)| VariableDeclaration {
                    declaration: Box::new(declaration),
                    variable: Box::new(variable),
                    token: 0,
                })
                .boxed()
        }
    }
}
