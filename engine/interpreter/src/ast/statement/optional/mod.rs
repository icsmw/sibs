#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Optional {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: ExecutionContext) -> RtPinnedResult<'_, LinkedErr<E>> {
        let comparison = self.comparison.interpret(rt.clone(), cx.clone()).await?;
        let RtValue::Bool(comparison) = comparison else {
            return Err(LinkedErr::from(
                E::InvalidType(Ty::Determined(DeterminedTy::Bool), comparison),
                &self.comparison,
            ));
        };
        if !comparison {
            return Ok(RtValue::Void);
        }
        self.action.interpret(rt, cx).await?;
        Ok(RtValue::Void)
    }
}
