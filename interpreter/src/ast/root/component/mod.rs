use crate::*;

impl Interpret for Component {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let (task, args) = rt.cx.get_task_params().await.map_err(LinkedErr::unlinked)?;
        let Some(task) = self.get_task(&task) else {
            return Err(LinkedErr::unlinked(E::TaskNotFound(task, self.get_name())));
        };
        task.interpret(rt).await
    }
}
