use crate::*;

declare_embedded_fn!(
    vec![(None, None, Ty::Repeated(DeterminedTy::Any))],
    DeterminedTy::Void
);

#[docs]
/// Documentation placeholder
#[boxed]
pub fn executor(env: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
    let FnEnv { args, .. } = env;
    for arg in args.iter() {
        println!("{:?}", arg.value);
    }
    Ok(RtValue::Void)
}
