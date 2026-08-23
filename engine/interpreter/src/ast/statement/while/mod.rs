#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for While {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let InterpreterEnvironment { cx, .. } = env.clone();
        cx.loops()
            .open(&self.uuid)
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        loop {
            if cx
                .loops()
                .is_stopped()
                .await
                .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?
            {
                break;
            }
            let vl = self.comparison.interpret(env.clone()).await?;
            let RtValue::Bool(vl) = vl else {
                return Err(LinkedErr::from(
                    E::InvalidValueType(format!("returns {vl} instead bool")),
                    &self.comparison,
                ));
            };
            if !vl {
                break;
            }
            self.block.interpret(env.clone()).await?;
        }
        cx.loops()
            .close()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        Ok(RtValue::Void)
    }
}
