use crate::*;

impl Interpret for Anchor {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let comp = rt
            .cx
            .get_target_component()
            .await
            .map_err(LinkedErr::unlinked)?;
        let Some(comp) = self.get_component(&comp) else {
            return Err(LinkedErr::unlinked(E::CompNotFound(comp)));
        };
        comp.interpret(rt).await
    }
}
