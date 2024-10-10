use super::SELF;
use crate::{
    elements::{Gatekeeper, Reference},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for Reference {}

impl TryExecute for Reference {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let target = cx.owner.ok_or(E::NoOwnerComponent.by(self))?;
            let (parent, task) = if self.path.len() == 1 {
                (target.as_component()?, &self.path[0])
            } else if self.path.len() == 2 {
                let parent = if self.path[0] == SELF {
                    target
                } else {
                    cx.components
                        .iter()
                        .find(|c| {
                            if let Ok(c) = c.as_component() {
                                c.name.to_string() == self.path[0]
                            } else {
                                false
                            }
                        })
                        .ok_or(E::NotFoundComponent(self.path[0].to_owned()).by(self))?
                };
                (parent.as_component()?, &self.path[1])
            } else {
                return Err(E::InvalidPartsInReference.by(self));
            };
            let scope = cx
                .cx
                .scope
                .create(format!("{}:{task}", parent.name), parent.get_cwd(&cx.cx)?)
                .await?;
            let (task_el, gatekeepers) = parent
                .get_task(task)
                .ok_or(E::TaskNotFound(task.to_owned(), parent.name.to_string()).by(self))?;
            let task = task_el.as_task()?;
            let mut args: Vec<Value> = Vec::new();
            for input in self.inputs.iter() {
                let output = input
                    .execute(cx.clone())
                    .await?
                    .not_empty_or(E::FailToGetAnyValueAsTaskArg.by(self))?;
                args.push(output);
            }
            let task_ref = task.as_reference(cx.clone().args(&args)).await?;
            let skippable =
                Gatekeeper::skippable(gatekeepers, &task_ref, cx.clone().sc(scope.clone())).await?;
            if skippable {
                cx.journal().debug(
                    task.get_name(),
                    format!("{task_ref} will be skipped because gatekeeper conclusion",),
                );
            }
            let result = if !skippable {
                task_el
                    .execute(cx.clone().args(&args).sc(scope.clone()))
                    .await
            } else {
                Ok(Value::empty())
            };
            scope.destroy().await?;
            result
        })
    }
}
