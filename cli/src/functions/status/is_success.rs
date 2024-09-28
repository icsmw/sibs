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
    _cx: Context,
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
        let Value::SpawnStatus(status) = &args[0].value else {
            return Err(LinkedErr::new(
                E::Executing(name(), "Expecting SpawnStatus type".to_owned()),
                Some(args_token),
            ))?;
        };
        Ok(Value::bool(status.success))
    })
}

#[cfg(test)]
mod tests {
    use crate::test_block;

    test_block!(
        expanded,
        r#"
            $status = `./target/debug/exit 0 100 200 10`;
            $status.is_success();
        "#,
        true
    );

    test_block!(
        short,
        r#"
            `./target/debug/exit 0 100 200 10`.is_success();
        "#,
        true
    );

    test_block!(
        short_condition,
        r#"
            $a = true;
            if $a {
                `./target/debug/exit 0 100 200 10`.is_success();
            };
        "#,
        true
    );

    test_block!(
        short_condition_in,
        r#"
            if `./target/debug/exit 0 100 200 10`.is_success() {
                true;
            };
        "#,
        true
    );

    test_block!(
        short_condition_mt,
        r#"
            $a = true;
            if $a {
                `./target/debug/exit 0 100 200 10`.is_success();
            } else {
                `./target/debug/exit 0 100 200 10`.is_success();
            };
        "#,
        true
    );
}
