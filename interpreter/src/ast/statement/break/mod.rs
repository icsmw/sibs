#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Break {
    #[boxed]
    fn interpret(&self, _rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        cx.loops()
            .set_break()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        Ok(RtValue::Void)
    }
}
