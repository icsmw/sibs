#[cfg(test)]
mod tests;

use std::ops::RangeInclusive;

use crate::*;

impl Interpret for Range {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        Ok(RtValue::Range(RangeInclusive::new(
            self.left
                .interpret(rt.clone(), cx.clone())
                .await?
                .try_to_rs()
                .map_err(|err| LinkedErr::from(err, &self.left))?,
            self.right
                .interpret(rt.clone(), cx.clone())
                .await?
                .try_to_rs()
                .map_err(|err| LinkedErr::from(err, &self.left))?,
        )))
    }
}
