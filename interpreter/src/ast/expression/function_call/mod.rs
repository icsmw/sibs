#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for FunctionCall {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let mut args = Vec::new();
        for n in self.args.iter() {
            args.push(n.interpret(rt.clone()).await?);
        }
        rt.clone().fns.execute(&self.uuid, rt, args).await
    }
}
