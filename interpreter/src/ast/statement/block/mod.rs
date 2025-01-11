use crate::*;

impl Interpret for Block {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        rt.scopes
            .enter(&self.uuid)
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        let mut last = None;
        for n in self.nodes.iter() {
            last = Some(n.interpret(rt.clone()).await?);
        }
        rt.scopes
            .leave()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        Ok(last.unwrap_or(RtValue::Void))
    }
}
