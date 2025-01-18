#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for CompoundAssignments {
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
        let Node::Expression(Expression::CompoundAssignmentsOp(op)) = &self.operator.node else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.operator.node.id()),
                &self.operator,
            ));
        };
        let left = rt
            .scopes
            .lookup(&variable)
            .await
            .map_err(|err| LinkedErr::from(err, &self.left))?
            .ok_or(LinkedErr::from(
                E::VariableNotFound(variable.clone()),
                &self.left,
            ))?;
        let right = self.right.interpret(rt.clone()).await?;
        chk_ty(&self.left, &right, &rt).await?;
        match &right {
            RtValue::Num(vl) => {
                let RtValue::Num(left) = left.as_ref() else {
                    return Err(LinkedErr::from(
                        E::InvalidValueType(right.id().to_string()),
                        &self.right,
                    ));
                };
                let updated = match op.operator {
                    CompoundAssignmentsOperator::MinusEqual => left - vl,
                    CompoundAssignmentsOperator::PlusEqual => left + vl,
                    CompoundAssignmentsOperator::SlashEqual => left / vl,
                    CompoundAssignmentsOperator::StarEqual => left * vl,
                };
                rt.scopes
                    .update(&variable, RtValue::Num(updated))
                    .await
                    .map_err(|err| LinkedErr::from(err, &self.right))?;
                Ok(RtValue::Num(updated))
            }
            RtValue::Str(vl) => {
                let RtValue::Str(left) = left.as_ref() else {
                    return Err(LinkedErr::from(
                        E::InvalidValueType(right.id().to_string()),
                        &self.right,
                    ));
                };
                let updated = match op.operator {
                    CompoundAssignmentsOperator::PlusEqual => format!("{left}{vl}"),
                    _ => {
                        return Err(LinkedErr::from(E::NotApplicableToTypeOperation, op));
                    }
                };
                rt.scopes
                    .update(&variable, RtValue::Str(updated.clone()))
                    .await
                    .map_err(|err| LinkedErr::from(err, &self.right))?;
                Ok(RtValue::Str(updated))
            }
            RtValue::PathBuf(vl) => {
                let RtValue::PathBuf(left) = left.as_ref() else {
                    return Err(LinkedErr::from(
                        E::InvalidValueType(right.id().to_string()),
                        &self.right,
                    ));
                };
                let updated = match op.operator {
                    CompoundAssignmentsOperator::PlusEqual => left.join(vl),
                    _ => {
                        return Err(LinkedErr::from(E::NotApplicableToTypeOperation, op));
                    }
                };
                rt.scopes
                    .update(&variable, RtValue::PathBuf(updated.clone()))
                    .await
                    .map_err(|err| LinkedErr::from(err, &self.right))?;
                Ok(RtValue::PathBuf(updated))
            }
            _ => Err(LinkedErr::from(
                E::InvalidValueType(right.id().to_string()),
                &self.right,
            )),
        }
    }
}
