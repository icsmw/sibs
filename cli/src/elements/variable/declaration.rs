use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, Formation,
        FormationCursor, LinkingResult, PrevValue, PrevValueExpectation, Scope, TokenGetter,
        TryExecute, TryExpectedValueType, Value, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub variable: Box<Element>,
    pub declaration: Box<Element>,
    pub token: usize,
}

impl VariableDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub async fn get_val<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> Result<Value, LinkedErr<operator::E>> {
        let input = if args.len() != 1 {
            Err(operator::E::InvalidNumberOfArgumentsForDeclaration)?
        } else {
            args[0].to_owned()
        };
        self.declaration
            .execute(
                owner,
                components,
                &[input.clone()],
                prev,
                cx.clone(),
                sc.clone(),
                token.clone(),
            )
            .await?
            .not_empty_or(
                operator::E::NoValueToDeclareTaskArgument.linked(&self.declaration.token()),
            )
    }
}

impl TryDissect<VariableDeclaration> for VariableDeclaration {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableDeclaration>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::VariableDeclaration);
        let Some(variable) = Element::include(reader, &[ElTarget::VariableName])? else {
            return Ok(None);
        };
        if reader.move_to().char(&[&chars::COLON]).is_none() {
            return Err(E::NoTypeDeclaration.by_reader(reader));
        }
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
    }
}

impl Dissect<VariableDeclaration, VariableDeclaration> for VariableDeclaration {}

impl TokenGetter for VariableDeclaration {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for VariableDeclaration {
    fn try_varification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }
    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            let Element::VariableName(el, _) = self.variable.as_ref() else {
                return Err(operator::E::NoVariableName.by(self));
            };
            cx.variables
                .set(
                    &owner.as_component()?.uuid,
                    el.get_name(),
                    self.declaration
                        .try_expected(owner, components, prev, cx)
                        .await?,
                )
                .await
                .map_err(|e| LinkedErr::new(e, Some(self.token)))?;
            Ok(())
        })
    }
    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move {
            self.declaration
                .try_expected(owner, components, prev, cx)
                .await
        })
    }
}

impl TryExecute for VariableDeclaration {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            sc.set_var(
                if let Element::VariableName(el, _) = self.variable.as_ref() {
                    &el.name
                } else {
                    Err(operator::E::FailToGetDeclaredVariable)?
                },
                self.get_val(owner, components, args, prev, cx, sc.clone(), token)
                    .await?,
            )
            .await?;
            Ok(Value::empty())
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
