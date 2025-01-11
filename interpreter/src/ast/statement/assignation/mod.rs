#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Assignation {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let variable = if let Node::Expression(Expression::Variable(variable)) = &self.left.node {
            variable.ident.to_owned()
        } else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.left.node.id()),
                &self.left,
            ));
        };
        let vl = self.right.interpret(rt.clone()).await?;
        chk_ty(&self.left, &vl, &rt).await?;
        rt.scopes
            .insert(&variable, vl)
            .await
            .map_err(|err| LinkedErr::from(err, &self.right))?;
        Ok(RtValue::Void)
    }
}
