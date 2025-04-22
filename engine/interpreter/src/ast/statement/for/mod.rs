#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for For {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let el = if let Node::Expression(Expression::Variable(el)) = self.element.get_node() {
            el.ident.to_owned()
        } else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.element.get_node().id()),
                &self.element,
            ));
        };
        let indx = if let Some(indx) = self.index.as_ref() {
            if let Node::Expression(Expression::Variable(indx)) = indx.get_node() {
                Some((indx.ident.to_owned(), indx))
            } else {
                return Err(LinkedErr::from(
                    E::UnexpectedNode(self.element.get_node().id()),
                    indx,
                ));
            }
        } else {
            None
        };
        let vls = match self.elements.interpret(rt.clone(), cx.clone()).await? {
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
        cx.loops()
            .open(&self.uuid)
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        for (n, vl) in vls.into_iter().enumerate() {
            if cx
                .loops()
                .is_stopped()
                .await
                .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?
            {
                break;
            }
            cx.values()
                .insert(&el, vl)
                .await
                .map_err(|err| LinkedErr::from(err, &self.element))?;
            if let Some((variable, node)) = indx.as_ref() {
                cx.values()
                    .insert(variable, RtValue::Num(n as f64))
                    .await
                    .map_err(|err| LinkedErr::from(err, *node))?;
            }
            self.block.interpret(rt.clone(), cx.clone()).await?;
        }
        cx.loops()
            .close()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.slink()).into()))?;
        Ok(RtValue::Void)
    }
}
