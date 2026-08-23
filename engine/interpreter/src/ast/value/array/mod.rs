use crate::*;

impl Interpret for Array {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let mut els = Vec::new();
        for el in self.els.iter() {
            els.push(el.interpret(env.clone()).await?);
        }
        Ok(RtValue::Vec(els))
    }
}
