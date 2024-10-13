use crate::{
    elements::Element,
    inf::{
        operator::E, Context, ExpectedResult, ExpectedValueType, LinkingResult,
        PrevValueExpectation, TryExpectedValueType, VerificationResult,
    },
};

impl TryExpectedValueType for Element {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            match self {
                Self::Call(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Accessor(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Function(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::If(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::IfCondition(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::IfSubsequence(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::IfThread(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Breaker(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Each(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::First(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Join(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::VariableAssignation(v, _) => {
                    v.try_verification(owner, components, prev, cx).await
                }
                Self::Comparing(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Combination(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Condition(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Subsequence(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Optional(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Gatekeeper(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Reference(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::PatternString(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::VariableName(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Values(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Block(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Command(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Task(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Component(v, _) => v.try_verification(self, components, prev, cx).await,
                Self::Integer(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Boolean(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::VariableDeclaration(v, _) => {
                    v.try_verification(owner, components, prev, cx).await
                }
                Self::VariableVariants(v, _) => {
                    v.try_verification(owner, components, prev, cx).await
                }
                Self::VariableType(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::SimpleString(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Range(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::For(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Compute(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Return(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Error(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Incrementer(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Loop(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::While(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Closure(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Conclusion(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Meta(..) => Err(E::NoReturnType.by(self)),
                Self::Comment(..) => Err(E::NoReturnType.by(self)),
            }
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
            match self {
                Self::Call(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Accessor(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Function(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::If(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::IfCondition(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::IfSubsequence(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::IfThread(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Breaker(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Each(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::First(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Join(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::VariableAssignation(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Comparing(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Combination(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Condition(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Subsequence(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Optional(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Gatekeeper(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Reference(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::PatternString(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::VariableName(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Values(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Block(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Command(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Task(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Component(v, _) => v.try_linking(self, components, prev, cx).await,
                Self::Integer(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Boolean(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::VariableDeclaration(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::VariableVariants(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::VariableType(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::SimpleString(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Range(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::For(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Compute(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Return(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Error(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Incrementer(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Loop(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::While(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Closure(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Conclusion(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Meta(..) => Err(E::NoReturnType.by(self)),
                Self::Comment(..) => Err(E::NoReturnType.by(self)),
            }
        })
    }
    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            match self {
                Self::Call(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Accessor(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Function(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::If(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::IfCondition(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::IfSubsequence(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::IfThread(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Breaker(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Each(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::First(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Join(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::VariableAssignation(v, _) => {
                    v.try_expected(owner, components, prev, cx).await
                }
                Self::Comparing(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Combination(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Condition(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Subsequence(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Optional(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Gatekeeper(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Reference(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::PatternString(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::VariableName(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Values(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Block(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Command(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Task(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Component(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Integer(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Boolean(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::VariableDeclaration(v, _) => {
                    v.try_expected(owner, components, prev, cx).await
                }
                Self::VariableVariants(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::VariableType(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::SimpleString(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Range(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::For(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Compute(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Return(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Error(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Incrementer(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Loop(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::While(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Closure(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Conclusion(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Meta(..) => Err(E::NoReturnType.by(self)),
                Self::Comment(..) => Err(E::NoReturnType.by(self)),
            }
        })
    }
}

impl ExpectedValueType for Element {}
