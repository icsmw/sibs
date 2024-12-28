use crate::*;

impl Interpret for AssignedValue {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let vl = self.node.interpret(rt.clone()).await?;
        chk_ty(&self.node, &vl, &rt).await?;
        Ok(vl)
    }
}
