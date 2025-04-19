use crate::*;

impl Interpret for Anchor {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let rt_params = rt
            .get_rt_parameters()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        let Some(component) = self.get_component(&rt_params.component) else {
            return Err(LinkedErr::from(E::CompNotFound(rt_params.component), self));
        };
        component.interpret(rt, cx).await
    }
}
