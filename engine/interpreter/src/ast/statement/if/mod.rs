#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for If {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        for case in self.cases.iter() {
            match case {
                IfCase::If(cnd, blk, _) => {
                    let RtValue::Bool(vl) = cnd.interpret(rt.clone(), cx.clone()).await? else {
                        return Err(LinkedErr::from(
                            E::InvalidValueType(RtValueId::Bool.to_string()),
                            cnd,
                        ));
                    };
                    if vl {
                        return blk.interpret(rt, cx).await;
                    }
                }
                IfCase::Else(blk, _) => {
                    return blk.interpret(rt, cx).await;
                }
            }
        }
        Ok(RtValue::Void)
    }
}
