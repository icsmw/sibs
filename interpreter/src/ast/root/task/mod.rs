#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Task {
    #[boxed]
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}

impl Execute for Task {
    #[boxed]
    fn exec(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        rt.evns
            .open_return_cx(&self.uuid)
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        let mut result = self.block.interpret(rt.clone()).await?;
        result = if let Some(result) = rt
            .evns
            .withdraw_return_vl(&self.uuid)
            .await
            .map_err(|err| LinkedErr::from(err, self))?
        {
            result
        } else {
            result
        };
        rt.evns
            .close_return_cx()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        Ok(result)
    }
}
