use crate::{
    elements::{Element, Task},
    executors::ExecutorPinnedResult,
    inf::{tools::get_name, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(task: &Task, args: &[Element], cx: Context, sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move { Ok(true) })
}
