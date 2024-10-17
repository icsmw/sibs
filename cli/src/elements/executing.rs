use crate::{
    elements::{Element, Metadata},
    error::LinkedErr,
    inf::*,
};

impl Processing for Element {
    fn processing<'a>(
        &'a self,
        results: &'a Value,
        cx: ExecuteContext<'a>,
    ) -> operator::ProcessingPinnedResult<'a> {
        Box::pin(async move {
            match self {
                Self::Conclusion(v, _) => v.processing(results, cx).await,
                Self::Closure(v, _) => v.processing(results, cx).await,
                Self::Loop(v, _) => v.processing(results, cx).await,
                Self::While(v, _) => v.processing(results, cx).await,
                Self::Incrementer(v, _) => v.processing(results, cx).await,
                Self::Return(v, _) => v.processing(results, cx).await,
                Self::Error(v, _) => v.processing(results, cx).await,
                Self::Compute(v, _) => v.processing(results, cx).await,
                Self::For(v, _) => v.processing(results, cx).await,
                Self::Range(v, _) => v.processing(results, cx).await,
                Self::Call(v, _) => v.processing(results, cx).await,
                Self::Accessor(v, _) => v.processing(results, cx).await,
                Self::Function(v, _) => v.processing(results, cx).await,
                Self::If(v, _) => v.processing(results, cx).await,
                Self::IfCondition(v, _) => v.processing(results, cx).await,
                Self::IfSubsequence(v, _) => v.processing(results, cx).await,
                Self::IfThread(v, _) => v.processing(results, cx).await,
                Self::Breaker(v, _) => v.processing(results, cx).await,
                Self::Each(v, _) => v.processing(results, cx).await,
                Self::First(v, _) => v.processing(results, cx).await,
                Self::Join(v, _) => v.processing(results, cx).await,
                Self::VariableAssignation(v, _) => v.processing(results, cx).await,
                Self::Comparing(v, _) => v.processing(results, cx).await,
                Self::Combination(v, _) => v.processing(results, cx).await,
                Self::Condition(v, _) => v.processing(results, cx).await,
                Self::Subsequence(v, _) => v.processing(results, cx).await,
                Self::Optional(v, _) => v.processing(results, cx).await,
                Self::Gatekeeper(v, _) => v.processing(results, cx).await,
                Self::Reference(v, _) => v.processing(results, cx).await,
                Self::PatternString(v, _) => v.processing(results, cx).await,
                Self::VariableName(v, _) => v.processing(results, cx).await,
                Self::Values(v, _) => v.processing(results, cx).await,
                Self::Block(v, _) => v.processing(results, cx).await,
                Self::Command(v, _) => v.processing(results, cx).await,
                Self::Task(v, _) => v.processing(results, cx).await,
                Self::Component(v, _) => v.processing(results, cx).await,
                Self::Integer(v, _) => v.processing(results, cx).await,
                Self::Boolean(v, _) => v.processing(results, cx).await,
                Self::VariableDeclaration(v, _) => v.processing(results, cx).await,
                Self::VariableVariants(v, _) => v.processing(results, cx).await,
                Self::VariableType(v, _) => v.processing(results, cx).await,
                Self::SimpleString(v, _) => v.processing(results, cx).await,
                Self::Meta(..) => Ok(()),
                Self::Comment(_) => Ok(()),
            }
        })
    }
}

impl TryExecute for Element {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let journal = cx.journal().clone();
            let result = match self {
                Self::Conclusion(v, _) => v.try_execute(cx.clone()).await,
                Self::Closure(v, _) => v.try_execute(cx.clone()).await,
                Self::Loop(v, _) => v.try_execute(cx.clone()).await,
                Self::While(v, _) => v.try_execute(cx.clone()).await,
                Self::Incrementer(v, _) => v.try_execute(cx.clone()).await,
                Self::Return(v, _) => v.try_execute(cx.clone()).await,
                Self::Error(v, _) => v.try_execute(cx.clone()).await,
                Self::Compute(v, _) => v.try_execute(cx.clone()).await,
                Self::For(v, _) => v.try_execute(cx.clone()).await,
                Self::Range(v, _) => v.try_execute(cx.clone()).await,
                Self::Call(v, _) => v.try_execute(cx.clone()).await,
                Self::Accessor(v, _) => v.try_execute(cx.clone()).await,
                Self::Function(v, _) => v.try_execute(cx.clone()).await,
                Self::If(v, _) => v.try_execute(cx.clone()).await,
                Self::IfCondition(v, _) => v.try_execute(cx.clone()).await,
                Self::IfSubsequence(v, _) => v.try_execute(cx.clone()).await,
                Self::IfThread(v, _) => v.try_execute(cx.clone()).await,
                Self::Breaker(v, _) => v.try_execute(cx.clone()).await,
                Self::Each(v, _) => v.try_execute(cx.clone()).await,
                Self::First(v, _) => v.try_execute(cx.clone()).await,
                Self::Join(v, _) => v.try_execute(cx.clone()).await,
                Self::VariableAssignation(v, _) => v.try_execute(cx.clone()).await,
                Self::Comparing(v, _) => v.try_execute(cx.clone()).await,
                Self::Combination(v, _) => v.try_execute(cx.clone()).await,
                Self::Condition(v, _) => v.try_execute(cx.clone()).await,
                Self::Subsequence(v, _) => v.try_execute(cx.clone()).await,
                Self::Optional(v, _) => v.try_execute(cx.clone()).await,
                Self::Gatekeeper(v, _) => v.try_execute(cx.clone()).await,
                Self::Reference(v, _) => v.try_execute(cx.clone()).await,
                Self::PatternString(v, _) => v.try_execute(cx.clone()).await,
                Self::VariableName(v, _) => v.try_execute(cx.clone()).await,
                Self::Values(v, _) => v.try_execute(cx.clone()).await,
                Self::Block(v, _) => v.try_execute(cx.clone()).await,
                Self::Command(v, _) => v.try_execute(cx.clone()).await,
                Self::Task(v, _) => v.try_execute(cx.clone()).await,
                Self::Component(v, _) => v.try_execute(cx.clone()).await,
                Self::Integer(v, _) => v.try_execute(cx.clone()).await,
                Self::Boolean(v, _) => v.try_execute(cx.clone()).await,
                Self::Meta(..) => Ok(Value::empty()),
                Self::VariableDeclaration(v, _) => v.try_execute(cx.clone()).await,
                Self::VariableVariants(v, _) => v.try_execute(cx.clone()).await,
                Self::VariableType(v, _) => v.try_execute(cx.clone()).await,
                Self::SimpleString(v, _) => v.try_execute(cx.clone()).await,
                Self::Comment(_) => Ok(Value::empty()),
            };
            if let (true, Err(err)) = (self.get_metadata().tolerance, result.as_ref()) {
                journal.as_tolerant(&err.uuid);
                return Ok(Value::empty());
            }
            let output = result?;
            Ok(if self.get_metadata().inverting {
                Value::bool(
                    !output
                        .not_empty_or(operator::E::InvertingOnEmptyReturn.by(self))?
                        .as_bool()
                        .ok_or(operator::E::InvertingOnNotBool.by(self))?,
                )
            } else {
                output
            })
        })
    }
}

impl Execute for Element {
    fn get_metadata(&self) -> Result<&Metadata, LinkedErr<operator::E>> {
        Ok(self.get_metadata())
    }
}
