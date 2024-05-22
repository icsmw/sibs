use crate::{
    functions::{get_name, ExecutorPinnedResult, E},
    inf::{AnyValue, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(args: Vec<AnyValue>, cx: Context, _sc: Scope) -> ExecutorPinnedResult {
    module_path!();
    Box::pin(async move {
        if args.len() != 2 {
            return Err(E::Executing(
                name(),
                "Expecting 2 income argument: varname, varvalue".to_owned(),
            ));
        }
        cx.scope
            .set_var(
                &args[0].as_string().ok_or(E::Executing(
                    name(),
                    "Cannot extract argument as string".to_owned(),
                ))?,
                args[1].duplicate(),
            )
            .await?;
        Ok(AnyValue::empty())
    })
}
