use crate::{
    functions::ExecutorPinnedResult,
    inf::{tools::get_name, AnyValue, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(msgs: Vec<AnyValue>, cx: Context, _sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move {
        for msg in msgs.iter() {
            cx.journal
                .warn(String::new(), msg.as_string().unwrap_or(format!("{msg:?}")));
        }
        Ok(AnyValue::empty())
    })
}
