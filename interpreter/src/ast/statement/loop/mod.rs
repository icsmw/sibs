#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Loop {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        cx.loops()
            .open(&self.uuid)
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        let mut vl = None;
        loop {
            if cx
                .loops()
                .is_stopped()
                .await
                .map_err(|err| LinkedErr::from(err, self))?
            {
                break;
            }
            vl = Some(self.block.interpret(rt.clone(), cx.clone()).await?);
        }
        cx.loops()
            .close()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        Ok(vl.unwrap_or(RtValue::Void))
    }
}
