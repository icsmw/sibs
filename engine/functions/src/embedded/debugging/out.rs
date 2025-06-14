use crate::*;

declare_embedded_fn!(
    vec![(None, None, Ty::Determined(DeterminedTy::Any))],
    DeterminedTy::Any
);

#[docs]
/// Documentation placeholder
#[boxed]
pub fn executor(
    args: Vec<FnArgValue>,
    _rt: Runtime,
    _cx: Context,
    caller: SrcLink,
) -> RtPinnedResult<'static, LinkedErr<E>> {
    let Some(arg) = args.first() else {
        return Err(LinkedErr::by_link(E::InvalidFnArgument, (&caller).into()));
    };
    Ok(arg.value.clone())
}
