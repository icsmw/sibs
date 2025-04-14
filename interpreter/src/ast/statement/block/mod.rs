use crate::*;

impl Interpret for Block {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        cx.location()
            .enter(&self.uuid)
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        let mut last = None;
        for n in self.nodes.iter() {
            if cx
                .loops()
                .is_stopped()
                .await
                .map_err(|err| LinkedErr::from(err, self))?
            {
                break;
            }
            last = Some(n.interpret(rt.clone(), cx.clone()).await?);
        }
        cx.location()
            .leave()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        Ok(last.unwrap_or(RtValue::Void))
    }
}
