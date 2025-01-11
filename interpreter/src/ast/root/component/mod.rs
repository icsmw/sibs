use crate::*;

impl Interpret for Component {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let (task, args) = rt
            .cx
            .get_task_params()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        let Some(task) = self.get_task(&task) else {
            return Err(LinkedErr::from(
                E::TaskNotFound(task, self.get_name()),
                self,
            ));
        };
        task.interpret(rt).await
    }
}
