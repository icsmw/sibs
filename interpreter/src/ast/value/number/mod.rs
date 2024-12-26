use crate::*;

impl Interpret for Number {
    #[boxed]
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        if let Kind::Number(n) = &self.token.kind {
            Ok(RtValue::Num(*n))
        } else {
            Err(LinkedErr::token(E::FailExtractValue, &self.token))
        }
    }
}
