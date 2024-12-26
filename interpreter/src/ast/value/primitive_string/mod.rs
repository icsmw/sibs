use crate::*;

impl Interpret for PrimitiveString {
    #[boxed]
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        if let Kind::String(s) = &self.token.kind {
            Ok(RtValue::Str(s.to_owned()))
        } else {
            Err(LinkedErr::token(E::FailExtractValue, &self.token))
        }
    }
}
