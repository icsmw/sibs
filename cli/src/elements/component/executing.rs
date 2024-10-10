use crate::{
    elements::{Component, Gatekeeper},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Component {}

impl TryExecute for Component {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let task = cx
                .args
                .first()
                .and_then(|task| task.as_string())
                .ok_or_else(|| {
                    E::NoTaskForComponent(self.name.to_string(), self.get_tasks_names())
                })?;
            let (task_el, gatekeepers) = self.get_task(&task).ok_or_else(|| {
                E::TaskNotExists(
                    self.name.to_string(),
                    task.to_owned(),
                    self.get_tasks_names(),
                )
            })?;
            let task = task_el.as_task()?;
            let sc = cx
                .cx
                .scope
                .create(
                    format!("{}:{}", self.name, task.get_name()),
                    self.cwd.clone(),
                )
                .await?;
            let inner = cx.clone().sc(sc.clone()).args(&cx.args[1..]);
            let task_ref = task.as_reference(inner.clone()).await?;
            let skippable = Gatekeeper::skippable(gatekeepers, &task_ref, cx.clone()).await?;
            if skippable {
                cx.journal().debug(
                    task.get_name(),
                    format!("{task_ref} will be skipped because gatekeeper conclusion",),
                );
            }
            let result = if !skippable {
                task_el.execute(inner.clone()).await
            } else {
                Ok(Value::Empty(()))
            };
            sc.destroy().await?;
            result
        })
    }
}
