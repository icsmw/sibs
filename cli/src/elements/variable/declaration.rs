use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult,
        Scope,
    },
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
    pub async fn get_val<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> Result<AnyValue, LinkedErr<operator::E>> {
        let input = if args.len() != 1 {
            Err(operator::E::InvalidNumberOfArgumentsForDeclaration)?
        } else {
            args[0].to_owned()
        };
        let mut output = if let Element::VariableType(el, _) = self.declaration.as_ref() {
            Some(
                el.execute(
                    owner,
                    components,
                    &[input.clone()],
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?,
            )
        } else {
            None
        };
        output = if let Element::VariableVariants(el, _) = self.declaration.as_ref() {
            Some(
                el.execute(owner, components, &[input], cx, sc.clone(), token)
                    .await?,
            )
        } else {
            output
        };
        Ok(output
            .ok_or(operator::E::FailToExtractValue)?
            .ok_or(operator::E::NoValueToDeclareTaskArgument)?)
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

impl Operator for VariableDeclaration {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            sc.set_var(
                if let Element::VariableName(el, _) = self.variable.as_ref() {
                    &el.name
                } else {
                    Err(operator::E::FailToGetDeclaredVariable)?
                },
                self.get_val(owner, components, args, cx, sc.clone(), token)
                    .await?,
            )
            .await?;
            Ok(None)
        })
    }
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.variable, self.declaration)
    }
}

impl Formation for VariableDeclaration {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        self.to_string()
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
