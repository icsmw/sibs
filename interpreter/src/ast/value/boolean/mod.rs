use crate::*;

impl Interpret for Boolean {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        if let Kind::Keyword(kw) = &self.token.kind {
            if matches!(kw, Keyword::True) {
                return Ok(RtValue::Bool(true));
            } else if matches!(kw, Keyword::False) {
                return Ok(RtValue::Bool(false));
            }
        }
        Err(LinkedErr::from(E::FailExtractValue, self))
    }
}
