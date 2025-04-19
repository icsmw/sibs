#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Return {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let vl = if let Some(n) = self.node.as_ref() {
            n.interpret(rt.clone(), cx.clone()).await?
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
