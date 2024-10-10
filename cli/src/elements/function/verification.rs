use crate::{
    elements::{function::Function, Element, TokenGetter},
    inf::{
        operator, Context, ExpectedResult, ExpectedValueType, HasOptional, HasRepeated,
        LinkingResult, PrevValueExpectation, TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Function {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            for el in self.args.iter() {
                el.verification(owner, components, prev, cx).await?;
            }
            let desc = cx
                .get_func_desc(&self.name, prev.as_ref().map(|v| v.value.clone()).clone())
                .await?;
            let ex_args = desc.args();
            let mut ac_args = Vec::new();
            for el in self.args.iter() {
                ac_args.push((el.expected(owner, components, prev, cx).await?, el.token()));
            }
            if let Some(prev) = prev {
                ac_args.insert(0, (prev.value.clone(), prev.token));
            }
            if ex_args.has_optional() && ex_args.has_repeated() {
                return Err(operator::E::RepeatedAndOptionalTypes(self.name.to_owned()).by(self));
            }
            if ex_args.has_optional() {
                if ex_args.len() < ac_args.len() {
                    return Err(operator::E::FunctionsArgsNumberNotMatch(
                        self.name.to_owned(),
                        ex_args.len(),
                        ac_args.len(),
                    )
                    .by(self));
                }
                for (n, (actual, actual_token)) in ac_args.iter().enumerate() {
                    if !ex_args[n].is_compatible(actual) {
                        return Err(operator::E::FunctionsArgNotMatchType(
                            self.name.to_owned(),
                            ex_args[n].to_owned(),
                            actual.to_owned(),
                        )
                        .linked(actual_token));
                    }
                }
            } else if ex_args.has_repeated() {
                if ex_args.len() > ac_args.len() {
                    return Err(operator::E::FunctionsArgsNumberNotMatch(
                        self.name.to_owned(),
                        ex_args.len(),
                        ac_args.len(),
                    )
                    .by(self));
                }
                let Some(ValueRef::Repeated(repeated)) = ex_args.last() else {
                    return Err(operator::E::InvalidRepeatedType(self.name.to_owned()).by(self));
                };
                for (n, (actual, actual_token)) in ac_args.iter().enumerate() {
                    if n < ex_args.len() - 1 && !ex_args[n].is_compatible(actual) {
                        return Err(operator::E::FunctionsArgNotMatchType(
                            self.name.to_owned(),
                            ex_args[n].to_owned(),
                            actual.to_owned(),
                        )
                        .linked(actual_token));
                    } else if repeated.is_compatible(actual) {
                        return Err(operator::E::FunctionsArgNotMatchType(
                            self.name.to_owned(),
                            *repeated.clone(),
                            actual.to_owned(),
                        )
                        .linked(actual_token));
                    }
                }
            } else {
                if ex_args.len() != ac_args.len() {
                    return Err(operator::E::FunctionsArgsNumberNotMatch(
                        self.name.to_owned(),
                        ex_args.len(),
                        ac_args.len(),
                    )
                    .by(self));
                }
                for (n, (actual, actual_token)) in ac_args.iter().enumerate() {
                    if !ex_args[n].is_compatible(actual) {
                        return Err(operator::E::FunctionsArgNotMatchType(
                            self.name.to_owned(),
                            ex_args[n].to_owned(),
                            actual.to_owned(),
                        )
                        .linked(actual_token));
                    }
                }
            }
            Ok(())
        })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move {
            for el in self.args.iter() {
                el.linking(owner, components, prev, cx).await?;
            }
            Ok(())
        })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            Ok(cx
                .get_func_desc(&self.name, prev.as_ref().map(|v| v.value.clone()).clone())
                .await?
                .output()?)
        })
    }
}
