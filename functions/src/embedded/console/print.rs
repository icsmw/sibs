use crate::*;

declare_embedded_fn!(Vec::new(), DataType::Void);

#[boxed]
pub fn executor(args: Vec<EmbeddedFnArg>, _rt: Runtime) -> RtPinnedResult<'static, LinkedErr<E>> {
    for arg in args.iter() {
        println!("{:?}", arg.value);
    }
    Ok(RtValue::Void)
}
