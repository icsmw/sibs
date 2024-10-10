use crate::{
    elements::{Element, Values},
    inf::{
        operator, Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Values {
    fn try_verification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { Ok(()) })
    }
    fn try_linking<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { Ok(()) })
    }
    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            let mut ty: Option<_> = None;
            for el in self.elements.iter() {
                if let Some(ty) = ty.as_ref() {
                    let current = el.expected(owner, components, prev, cx).await?;
                    if !current.is_compatible(ty) {
                        return Err(operator::E::DismatchTypesInVector(
                            ty.to_string(),
                            current.to_string(),
                        )
                        .by(el));
                    }
                } else {
                    ty = Some(el.expected(owner, components, prev, cx).await?)
                }
            }
            Ok(ValueRef::Vec(Box::new(ty.ok_or(operator::E::EmptyVector)?)))
        })
    }
}
