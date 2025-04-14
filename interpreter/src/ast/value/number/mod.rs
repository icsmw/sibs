use crate::*;

impl Interpret for Number {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        if let Kind::Number(n) = &self.token.kind {
            Ok(RtValue::Num(*n))
        } else {
            Err(LinkedErr::from(E::FailExtractValue, self))
        }
    }
}
