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
                E::Executing(
                    name(),
                    "Expecting 1 income argument: varname, varvalue".to_owned(),
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
        Ok(Value::usize(els.len()))
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
                if $el != "two" {
                    true;
                } else {
                    false;
                };
            });
            if $filtered.vec::len() == 2 && $filtered[0] == "one" && $filtered[1] == "three" {
                true;
            } else {
                false;
            };
        "#,
        true
    );

    test_block!(
        iteration_with_shortnames,
        r#"
            $els = ("one", "two", "three");
            $filtered = $els.filter(($n, $el) {
                $el != "two";
            });
            $filtered.len() == 2 && $filtered[0] == "one" && $filtered[1] == "three";
        "#,
        true
    );
}
