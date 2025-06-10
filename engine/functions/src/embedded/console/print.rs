use crate::*;

declare_embedded_fn!(
    vec![(None, None, Ty::Repeated(DeterminedTy::Any))],
    DeterminedTy::Void
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
    for arg in args.iter() {
        println!("{:?}", arg.value);
    }
    Ok(RtValue::Void)
}
