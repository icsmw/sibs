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
    _sc: Scope,
) -> ExecutorPinnedResult {
    module_path!();
    Box::pin(async move {
        if args.len() != 1 {
            Err(LinkedErr::new(
                E::Executing(name(), "Expecting 1 income argument".to_owned()),
                Some(args_token),
            ))?;
        }
        let Value::SpawnStatus(status) = args[0].value.clone() else {
            return Err(LinkedErr::new(
                E::Executing(name(), "Expecting SpawnStatus type".to_owned()),
                Some(args_token),
            ))?;
        };
        if !status.success {
            cx.journal.err("Command", format!("\n{}", status.report()));
            cx.abort(status.code.unwrap_or(1), status.error.clone())
                .await?;
        }
        Ok(Value::SpawnStatus(status))
    })
}

#[cfg(test)]
mod tests {
    use crate::{inf::Value, test_block};

    test_block!(
        short,
        r#"
            `./target/debug/exit 1 100 200 10`.stop_on_fail();
            true;
        "#,
        Value::Empty(())
    );

    test_block!(
        assignation,
        r#"
            $status = `./target/debug/exit 1 100 200 10`.stop_on_fail();
            true;
        "#,
        Value::Empty(())
    );

    test_block!(
        condition,
        r#"
            if `./target/debug/exit 1 100 200 10`.stop_on_fail().is_success() == true {
                true;
            };
            true;
        "#,
        Value::Empty(())
    );
}
