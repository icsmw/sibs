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
        iteration,
        r#"
            $els = ("one", "two", "three");
            $filtered = $els.vec::filter(($n, $el) {
                $el != "two";
            });
            $filtered.vec::len();
        "#,
        2usize
    );

    test_block!(
        iteration_short_name,
        r#"
            $els = ("one", "two", "three");
            $filtered = $els.filter(($n, $el) {
                $el != "two";
            });
            $filtered.len();
        "#,
        2usize
    );
}
