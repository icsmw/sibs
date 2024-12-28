use crate::*;

impl Interpret for VariableDeclaration {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let variable =
            if let Node::Declaration(Declaration::VariableName(variable)) = &self.variable.node {
                variable.ident.to_owned()
            } else {
                return Err(LinkedErr::by_node(
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
                .map_err(|err| LinkedErr::by_node(err.into(), node))?;
        }
        Ok(RtValue::Void)
    }
}
