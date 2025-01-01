#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for FunctionCall {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let mut args = Vec::new();
        if let Some(parent) = rt
            .scopes
            .withdraw_parent_vl()
            .await
            .map_err(|err| LinkedErr::between(err, &self.open, &self.close))?
        {
            args.push(parent.into());
        }
        for n in self.args.iter() {
            args.push(FnArgValue::by_node(n.interpret(rt.clone()).await?, n));
        }
        rt.clone().fns.execute(&self.uuid, rt, args).await
    }
}
