#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for If {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        for case in self.cases.iter() {
            match case {
                IfCase::If(cnd, blk, _) => {
                    let RtValue::Bool(vl) = cnd.interpret(env.clone()).await? else {
                        return Err(LinkedErr::from(
                            E::InvalidValueType(RtValueId::Bool.to_string()),
                            cnd,
                        ));
                    };
                    if vl {
                        return blk.interpret(env.clone()).await;
                    }
                }
                IfCase::Else(blk, _) => {
                    return blk.interpret(env.clone()).await;
                }
            }
        }
        Ok(RtValue::Void)
    }
}
