#[cfg(test)]
mod tests;

use crate::*;

fn apply_operator(
    left: &RtValue,
    operator: &CompoundAssignmentsOperator,
    right: &RtValue,
) -> Result<RtValue, E> {
    match (left, right) {
        (RtValue::Num(left), RtValue::Num(right)) => Ok(RtValue::Num(match operator {
            CompoundAssignmentsOperator::MinusEqual => left - right,
            CompoundAssignmentsOperator::PlusEqual => left + right,
            CompoundAssignmentsOperator::SlashEqual => left / right,
            CompoundAssignmentsOperator::StarEqual => left * right,
        })),
        (RtValue::Str(left), RtValue::Str(right)) => match operator {
            CompoundAssignmentsOperator::PlusEqual => Ok(RtValue::Str(format!("{left}{right}"))),
            CompoundAssignmentsOperator::MinusEqual
            | CompoundAssignmentsOperator::SlashEqual
            | CompoundAssignmentsOperator::StarEqual => Err(E::NotApplicableToTypeOperation),
        },
        (RtValue::PathBuf(left), RtValue::PathBuf(right)) => match operator {
            CompoundAssignmentsOperator::PlusEqual => Ok(RtValue::PathBuf(left.join(right))),
            CompoundAssignmentsOperator::MinusEqual
            | CompoundAssignmentsOperator::SlashEqual
            | CompoundAssignmentsOperator::StarEqual => Err(E::NotApplicableToTypeOperation),
        },
        _ => Err(E::InvalidValueType(left.id().to_string())),
    }
}

impl Interpret for CompoundAssignments {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let InterpreterEnvironment { rt, cx, .. } = env.clone();
        let variable =
            if let Node::Expression(Expression::Variable(variable)) = self.left.get_node() {
                variable.ident.to_owned()
            } else {
                return Err(LinkedErr::from(
                    E::UnexpectedNode(self.left.get_node().id()),
                    &self.left,
                ));
            };
        let Node::Expression(Expression::CompoundAssignmentsOp(op)) = self.operator.get_node()
        else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.operator.get_node().id()),
                &self.operator,
            ));
        };
        let left = cx
            .values()
            .lookup(&variable)
            .await
            .map_err(|err| LinkedErr::from(err, &self.left))?;
        let right = self.right.interpret(env.clone()).await?;
        chk_ty(&self.left, &right, &rt).await?;
        if let Some(left) = left {
            let updated = apply_operator(&left, &op.operator, &right)
                .map_err(|err| LinkedErr::from(err, &self.operator))?;
            cx.values()
                .update(&variable, updated.clone())
                .await
                .map_err(|err| LinkedErr::from(err, &self.left))?;
            Ok(updated)
        } else {
            // The right-hand expression is evaluated once before sending the transformation.
            let operator = op.operator.clone();
            let updated = cx
                .globals()
                .update(&variable, move |left| {
                    apply_operator(left, &operator, &right)
                })
                .await
                .map_err(|err| LinkedErr::from(err, &self.left))?;
            Ok((*updated).clone())
        }
    }
}
