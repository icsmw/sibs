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
        if args.len() != 2 {
            Err(LinkedErr::new(
                E::Executing(
                    name(),
                    "Expecting 2 income argument: varname, varvalue".to_owned(),
                ),
                Some(args_token),
            ))?;
        }
        sc.set_global_var(
            &args[0].value.as_string().ok_or(E::Executing(
                name(),
                "Cannot extract argument as string".to_owned(),
            ))?,
            args[1].value.duplicate(),
        )
        .await?;
        Ok(Value::empty())
    })
}
