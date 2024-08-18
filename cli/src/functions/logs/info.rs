use crate::{
    elements::FuncArg,
    functions::ExecutorPinnedResult,
    inf::{tools::get_name, Value, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(
    msgs: Vec<FuncArg>,
    _args_token: usize,
    cx: Context,
    _sc: Scope,
) -> ExecutorPinnedResult {
    Box::pin(async move {
        for msg in msgs.iter() {
            cx.journal.info(
                String::new(),
                msg.value.as_string().unwrap_or(format!("{msg:?}")),
            );
        }
        Ok(Value::empty())
    })
}
