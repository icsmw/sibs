use crate::*;

declare_embedded_fn!(
    vec![(None, None, Ty::Repeated(DeterminedTy::Num))],
    DeterminedTy::Num
);

#[docs]
/// Documentation placeholder
#[boxed]
pub fn executor(env: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
    let FnEnv { args, .. } = env;
    let mut sum: f64 = 0.0;
    for arg in args.iter() {
        if let RtValue::Num(vl) = arg.value {
            sum += vl;
        } else {
            return Err(LinkedErr::by_link(
                E::InvalidValueType(RtValueId::Num.to_string()),
                (&arg.link).into(),
            ));
        }
    }
    Ok(RtValue::Num(sum))
}
