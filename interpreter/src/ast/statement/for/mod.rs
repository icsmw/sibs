#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for For {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let el = if let Node::Expression(Expression::Variable(el)) = &self.element.node {
            el.ident.to_owned()
        } else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.element.node.id()),
                &self.element,
            ));
        };
        let indx = if let Some(indx) = self.index.as_ref() {
            if let Node::Expression(Expression::Variable(indx)) = &indx.node {
                Some((indx.ident.to_owned(), indx))
            } else {
                return Err(LinkedErr::from(
                    E::UnexpectedNode(self.element.node.id()),
                    indx,
                ));
            }
        } else {
            None
        };
        let vls = match self.elements.interpret(rt.clone()).await? {
            RtValue::Vec(els) => els,
            RtValue::Range(range) => range
                .collect::<Vec<isize>>()
                .into_iter()
                .map(|vl| RtValue::Num(vl as f64))
                .collect::<Vec<RtValue>>(),
            RtValue::Str(s) => s
                .chars()
                .map(|c| RtValue::Str(c.to_string()))
                .collect::<Vec<RtValue>>(),
            _ => {
                return Err(LinkedErr::from(E::InvalidIterationSource, &self.elements));
            }
        };
        rt.evns
            .open_loop(&self.uuid)
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        for (n, vl) in vls.into_iter().enumerate() {
            if rt
                .evns
                .is_stopped()
                .await
                .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?
            {
                break;
            }
            rt.scopes
                .insert(&el, vl)
                .await
                .map_err(|err| LinkedErr::from(err, &self.element))?;
            if let Some((variable, node)) = indx.as_ref() {
                rt.scopes
                    .insert(variable, RtValue::Num(n as f64))
                    .await
                    .map_err(|err| LinkedErr::from(err, *node))?;
            }
            self.block.interpret(rt.clone()).await?;
        }
        rt.evns
            .close_loop()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        Ok(RtValue::Void)
    }
}
