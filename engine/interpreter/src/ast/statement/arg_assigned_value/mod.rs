use crate::*;

impl Interpret for ArgumentAssignedValue {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let vl = self.node.interpret(rt.clone(), cx.clone()).await?;
        chk_ty(&self.node, &vl, &rt).await?;
        Ok(vl)
    }
}
