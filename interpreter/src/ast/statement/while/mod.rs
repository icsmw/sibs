#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for While {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        rt.evns
            .open_loop(&self.uuid)
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        loop {
            if rt
                .evns
                .is_stopped()
                .await
                .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?
            {
                break;
            }
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
        rt.evns
            .close_loop()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        Ok(RtValue::Void)
    }
}
