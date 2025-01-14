use crate::*;

impl Interpret for VariableDeclaration {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let variable =
            if let Node::Declaration(Declaration::VariableName(variable)) = &self.variable.node {
                variable.ident.to_owned()
            } else {
                return Err(LinkedErr::from(
                    E::UnexpectedNode(self.variable.node.id()),
                    &self.variable,
                ));
            };
        if let Some(node) = &self.assignation {
            let vl = node.interpret(rt.clone()).await?;
            if let Some(ty) = &self.r#type {
                chk_ty(ty, &vl, &rt).await?;
            }
            rt.scopes
                .insert(&variable, vl)
                .await
                .map_err(|err| LinkedErr::from(err, node))?;
        } else {
            rt.scopes
                .insert(&variable, RtValue::Void)
                .await
                .map_err(|err| LinkedErr::from(err, &self.variable))?;
        }
        Ok(RtValue::Void)
    }
}
