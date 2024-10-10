mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;
use crate::{
    elements::{Element, TokenGetter},
    error::LinkedErr,
    inf::{operator::E, Execute, ExecuteContext, Value},
};

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub variable: Box<Element>,
    pub declaration: Box<Element>,
    pub token: usize,
}

impl VariableDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub async fn get_val<'a>(&'a self, cx: ExecuteContext<'a>) -> Result<Value, LinkedErr<E>> {
        let input = if cx.args.len() != 1 {
            Err(E::InvalidNumberOfArgumentsForDeclaration)?
        } else {
            cx.args[0].to_owned()
        };
        self.declaration
            .execute(cx.clone().args(&[input.clone()]))
            .await?
            .not_empty_or(E::NoValueToDeclareTaskArgument.linked(&self.declaration.token()))
    }
}
