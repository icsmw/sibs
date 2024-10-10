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
    inf::{operator, Execute, ExecuteContext, Value},
    reader::E,
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct FuncArg {
    pub value: Value,
    pub token: usize,
}

impl FuncArg {
    pub fn new(value: Value, token: usize) -> Self {
        Self { value, token }
    }
    pub fn err<T: Clone + fmt::Display>(&self, err: T) -> LinkedErr<T> {
        LinkedErr::new(err, Some(self.token))
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<Element>,
    pub token: usize,
    pub args_token: usize,
}

impl Function {
    pub fn new(
        token: usize,
        args_token: usize,
        args: Vec<Element>,
        name: String,
    ) -> Result<Self, LinkedErr<E>> {
        Ok(Self {
            token,
            args_token,
            name,
            args,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_processed_args<'a>(
        &self,
        cx: ExecuteContext<'a>,
    ) -> Result<Vec<FuncArg>, operator::E> {
        let mut values: Vec<FuncArg> = Vec::new();
        for arg in self.args.iter() {
            values.push(FuncArg::new(arg.execute(cx.clone()).await?, arg.token()))
        }
        if let Some(prev) = cx.prev {
            values.insert(0, FuncArg::new(prev.value.duplicate(), prev.token))
        }
        Ok(values)
    }
}
