#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Return {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let InterpreterEnvironment { cx, .. } = env.clone();
        let vl = if let Some(n) = self.node.as_ref() {
            n.interpret(env.clone()).await?
        } else {
            RtValue::Void
        };
        cx.returns()
            .set_vl(vl)
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        Ok(RtValue::Void)
    }
}
