use crate::*;

impl Interpret for ArgumentAssignation {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let variable = if let Node::Expression(Expression::Variable(variable)) = &self.left.node {
            variable.ident.to_owned()
        } else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.left.node.id()),
                &self.left,
            ));
        };
        let vl = self.right.interpret(rt.clone(), cx.clone()).await?;
        Ok(RtValue::NamedArgumentValue(variable, Box::new(vl)))
    }
}
