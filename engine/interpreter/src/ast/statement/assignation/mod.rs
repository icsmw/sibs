#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Assignation {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let variable =
            if let Node::Expression(Expression::Variable(variable)) = self.left.get_node() {
                variable.ident.to_owned()
            } else {
                return Err(LinkedErr::from(
                    E::UnexpectedNode(self.left.get_node().id()),
                    &self.left,
                ));
            };
        let vl = self.right.interpret(rt.clone(), cx.clone()).await?;
        chk_ty(&self.left, &vl, &rt).await?;
        cx.values()
            .update(&variable, vl)
            .await
            .map_err(|err| LinkedErr::from(err, &self.right))?;
        Ok(RtValue::Void)
    }
}
