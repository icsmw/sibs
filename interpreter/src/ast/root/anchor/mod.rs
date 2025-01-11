use crate::*;

impl Interpret for Anchor {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let comp = rt
            .cx
            .get_target_component()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        let Some(comp) = self.get_component(&comp) else {
            return Err(LinkedErr::from(E::CompNotFound(comp), self));
        };
        comp.interpret(rt).await
    }
}
