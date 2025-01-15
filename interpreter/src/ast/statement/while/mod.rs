#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for While {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        loop {
            let vl = self.comparison.interpret(rt.clone()).await?;
            let RtValue::Bool(vl) = vl else {
                return Err(LinkedErr::from(
                    E::InvalidValueType(format!("returns {vl} instead bool")),
                    &self.comparison,
                ));
            };
            if !vl {
                break;
            }
            self.block.interpret(rt.clone()).await?;
        }
        Ok(RtValue::Void)
    }
}
