use crate::{
    executors::{get_name, ExecutorPinnedResult, E},
    inf::{AnyValue, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(args: Vec<AnyValue>, cx: Context, _sc: Scope) -> ExecutorPinnedResult {
    module_path!();
    Box::pin(async move {
        if args.len() != 1 {
            return Err(E::Executing(
                name(),
                "Expecting 1 income argument: varname".to_owned(),
            ));
        }
        Ok(cx
            .scope
            .get_var(&args[0].as_string().ok_or(E::Executing(
                name(),
                "Cannot extract argument as string".to_owned(),
            ))?)
            .await?
            .map(|v| v.duplicate())
            .unwrap_or(AnyValue::empty()))
    })
}
