use crate::{
    elements::FuncArg,
    error::LinkedErr,
    functions::{ExecutorPinnedResult, E},
    inf::{tools::get_name, Value, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(
    args: Vec<FuncArg>,
    args_token: usize,
    _cx: Context,
    sc: Scope,
) -> ExecutorPinnedResult {
    module_path!();
    Box::pin(async move {
        if args.len() != 1 {
            Err(LinkedErr::new(
                E::Executing(name(), "Expecting 1 income argument: varname".to_owned()),
                Some(args_token),
            ))?;
        }
        Ok(sc
            .get_var(&args[0].value.as_string().ok_or(E::Executing(
                name(),
                "Cannot extract argument as string".to_owned(),
            ))?)
            .await?
            .map(|v| v.duplicate())
            .unwrap_or(Value::empty()))
    })
}
