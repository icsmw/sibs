use crate::*;

impl Interpret for VariableDeclaration {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let variable = if let Node::Declaration(Declaration::VariableName(variable)) =
            self.variable.get_node()
        {
            variable.ident.to_owned()
        } else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.variable.get_node().id()),
                &self.variable,
            ));
        };
        if let Some(node) = &self.assignation {
            let vl = node.interpret(rt.clone(), cx.clone()).await?;
            if let Some(ty) = &self.r#type {
                chk_ty(ty, &vl, &rt).await?;
            }
            cx.values()
                .insert(&variable, vl)
                .await
                .map_err(|err| LinkedErr::from(err, node))?;
        } else {
            cx.values()
                .insert(&variable, RtValue::Void)
                .await
                .map_err(|err| LinkedErr::from(err, &self.variable))?;
        }
        Ok(RtValue::Void)
    }
}
