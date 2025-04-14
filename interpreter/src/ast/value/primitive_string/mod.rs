use crate::*;

impl Interpret for PrimitiveString {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        if let Kind::String(s) = &self.token.kind {
            Ok(RtValue::Str(s.to_owned()))
        } else {
            Err(LinkedErr::from(E::FailExtractValue, self))
        }
    }
}
