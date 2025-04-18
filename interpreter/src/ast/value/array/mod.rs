use crate::*;

impl Interpret for Array {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let mut els = Vec::new();
        for el in self.els.iter() {
            els.push(el.interpret(rt.clone(), cx.clone()).await?);
        }
        Ok(RtValue::Vec(els))
    }
}
