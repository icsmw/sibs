#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Loop {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        rt.evns
            .open_loop(&self.uuid)
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        let mut vl = None;
        loop {
            if rt
                .evns
                .is_stopped()
                .await
                .map_err(|err| LinkedErr::from(err, self))?
            {
                break;
            }
            vl = Some(self.block.interpret(rt.clone()).await?);
        }
        rt.evns
            .close_loop()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        Ok(vl.unwrap_or(RtValue::Void))
    }
}
