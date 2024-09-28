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
        let Value::Vec(statuses) = &args[0].value else {
            return Err(LinkedErr::new(
                E::Executing(name(), "Expecting Vec<SpawnStatus> type".to_owned()),
                Some(args_token),
            ))?;
        };
        for status in statuses {
            let Value::SpawnStatus(status) = status else {
                return Err(LinkedErr::new(
                    E::Executing(name(), "Expecting Vec<SpawnStatus> type".to_owned()),
                    Some(args_token),
                ))?;
            };
            if !status.success {
                return Ok(Value::bool(true));
            }
        }
        Ok(Value::bool(false))
    })
}

#[cfg(test)]
mod tests {
    use crate::test_block;

    test_block!(
        expanded_one,
        r#"
            $status = join (
                `./target/debug/exit 0 100 1000 10`,
                `./target/debug/exit 0 200 1000 10`,
                `./target/debug/exit 1 300 1000 10`,
                `./target/debug/exit 0 400 1000 10`,
            );
            $status.is_any_fail();
        "#,
        true
    );

    test_block!(
        expanded_two,
        r#"
            $status = join (
                `./target/debug/exit 0 100 1000 10`,
                `./target/debug/exit 1 200 1000 10`,
                `./target/debug/exit 1 300 1000 10`,
                `./target/debug/exit 0 400 1000 10`,
            );
            $status.is_any_fail();
        "#,
        true
    );
    test_block!(
        short_three,
        r#"
            join (
                `./target/debug/exit 1 100 1000 10`,
                `./target/debug/exit 1 200 1000 10`,
                `./target/debug/exit 1 300 1000 10`,
                `./target/debug/exit 0 400 1000 10`,
            ).is_any_fail();
        "#,
        true
    );

    test_block!(
        short_all,
        r#"
            join (
                `./target/debug/exit 1 100 1000 10`,
                `./target/debug/exit 1 200 1000 10`,
                `./target/debug/exit 1 300 1000 10`,
                `./target/debug/exit 1 400 1000 10`,
            ).is_any_fail();
        "#,
        true
    );
}
