#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Optional {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let comparison = self.comparison.interpret(env.clone()).await?;
        let RtValue::Bool(comparison) = comparison else {
            return Err(LinkedErr::from(
                E::InvalidType(Ty::Determined(DeterminedTy::Bool), comparison),
                &self.comparison,
            ));
        };
        if !comparison {
            return Ok(RtValue::Void);
        }
        self.action.interpret(env).await?;
        Ok(RtValue::Void)
    }
}
