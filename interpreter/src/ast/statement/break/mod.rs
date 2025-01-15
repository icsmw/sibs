#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Break {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        rt.evns
            .set_break()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        Ok(RtValue::Void)
    }
}
