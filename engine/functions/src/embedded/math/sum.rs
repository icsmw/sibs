use crate::*;

declare_embedded_fn!(
    vec![(None, None, Ty::Repeated(DeterminedTy::Num))],
    DeterminedTy::Num
);

#[docs]
/// Documentation placeholder
#[boxed]
pub fn executor(
    args: Vec<FnArgValue>,
    _rt: Runtime,
    _cx: Context,
    _caller: SrcLink,
) -> RtPinnedResult<'static, LinkedErr<E>> {
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
