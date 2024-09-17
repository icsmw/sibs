use tokio_util::sync::CancellationToken;

use crate::{
    elements::FuncArg,
    error::LinkedErr,
    functions::{ExecutorPinnedResult, E},
    inf::{tools::get_name, Context, Scope, Value},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(
    args: Vec<FuncArg>,
    args_token: usize,
    cx: Context,
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
        let Value::Vec(els) = &args[0].value else {
            return Err(LinkedErr::new(
                E::Executing(name(), "Expecting Vector as the first argument".to_owned()),
                Some(args_token),
            ))?;
        };
        let Value::Closure(uuid) = &args[1].value else {
            return Err(LinkedErr::new(
                E::Executing(name(), "Expecting Vector as the first argument".to_owned()),
                Some(args_token),
            ))?;
        };
        let closure = cx.closures.get(*uuid).await?;
        let vars = closure.get_vars_names();
        if vars.len() != 2 {
            return Err(E::InvalidClosureArgumentsCount(2, vars.len()).by(&closure));
        }
        for (n, var) in vars.iter().enumerate() {
            if sc.get_var(var).await?.is_some() {
                return Err(E::ClosureVariableIsUsed(var.to_owned()).by(&closure.args[n]));
            }
        }
        let mut output = Vec::new();
        for (n, el) in els.iter().enumerate() {
            sc.set_var(&vars[0], Value::usize(n)).await?;
            sc.set_var(&vars[1], el.duplicate()).await?;
            output.push(
                closure
                    .execute_block(cx.clone(), sc.clone(), CancellationToken::new())
                    .await?,
            );
        }
        Ok(Value::Vec(output))
    })
}
