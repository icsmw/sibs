use crate::*;

declare_embedded_fn!(
    vec![Ty::Determinated(DeterminatedTy::Str)],
    DeterminatedTy::Void
);

#[boxed]
pub fn executor(args: Vec<FnArgValue>, _rt: Runtime) -> RtPinnedResult<'static, LinkedErr<E>> {
    for arg in args.iter() {
        println!("{:?}", arg.value);
    }
    Ok(RtValue::Void)
}
