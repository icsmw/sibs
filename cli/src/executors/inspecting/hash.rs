use crate::{
    elements::{Element, Task},
    executors::ExecutorPinnedResult,
    inf::{tools::get_name, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(task: &Task, args: &[Element], cx: Context, sc: Scope) -> ExecutorPinnedResult {
    println!(
        ">>>>>>>>>>>>>>>>>>>>>>>>> INSIDE INSPECT GATEKEEPER TASK:{:?}",
        task
    );
    println!(
        ">>>>>>>>>>>>>>>>>>>>>>>>> INSIDE INSPECT GATEKEEPER ARGS:{:?}",
        args
    );
    let Element::Values(tasks, md) = args.get(1).unwrap() else {
        return Box::pin(async move { Ok(true) });
    };
    let Element::Reference(task_ref, md) = tasks.elements.first().unwrap() else {
        return Box::pin(async move { Ok(true) });
    };
    println!(
        ">>>>>>>>>>>>>>>>>>>>>>>>> INSIDE INSPECT GATEKEEPER TASK_REF:{:?}",
        task_ref
    );
    Box::pin(async move { Ok(true) })
}
