use crate::*;

impl Interpret for Component {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let rt_params = rt
            .get_rt_parameters()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        rt.clone()
            .tasks
            .execute_by_name(
                &self.uuid,
                rt_params.task,
                rt,
                cx,
                rt_params
                    .args
                    .into_iter()
                    .map(|arg| FnArgValue::new(RtValue::Str(arg), SrcLink::default()))
                    .collect(),
                &self.link(),
            )
            .await
    }
}
