use crate::*;

impl Interpret for Component {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let (task, args) = rt
            .cx
            .get_task_params()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        rt.clone()
            .tasks
            .execute_by_name(&self.uuid, task, rt, vec![], &self.link())
            .await
    }
}
