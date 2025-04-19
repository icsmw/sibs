#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Skip {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        for arg in self.args.iter() {
            let value = arg.interpret(rt.clone(), cx.clone()).await?;
            let RtValue::NamedArgumentValue(name, expected) = value else {
                return Err(LinkedErr::from(
                    E::DismatchValueType(
                        RtValueId::NamedArgumentValue.to_string(),
                        value.id().to_string(),
                    ),
                    arg,
                ));
            };
            let actual = cx
                .values()
                .lookup(name)
                .await
                .map_err(|err| LinkedErr::from(err, arg))?;
            let Some(actual) = actual else {
                return Ok(RtValue::Bool(true));
            };
            if let (Some(actual_ty), Some(expected_ty)) = (actual.as_ty(), expected.as_ty()) {
                if actual_ty != expected_ty {
                    return Err(LinkedErr::from(
                        E::DismatchValueType(expected_ty.to_string(), actual_ty.to_string()),
                        arg,
                    ));
                }
            } else {
                return Err(LinkedErr::from(
                    E::DismatchValueType(expected.id().to_string(), actual.id().to_string()),
                    arg,
                ));
            }
            if actual != expected.into() {
                return Ok(RtValue::Bool(true));
            }
        }
        let value = self.func.interpret(rt, cx).await?;
        if let RtValue::Bool(value) = value {
            Ok(RtValue::Bool(value))
        } else {
            Err(LinkedErr::from(
                E::DismatchValueType(RtValueId::Bool.to_string(), value.id().to_string()),
                &self.func,
            ))
        }
    }
}
