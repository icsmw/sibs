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
    elements::Element,
    inf::{Context, Execute, ExecuteContext, ExecutePinnedResult, Scope},
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Closure {
    pub args: Vec<Element>,
    pub block: Box<Element>,
    pub token: usize,
    pub uuid: Uuid,
}

impl Closure {
    pub fn get_vars_names(&self) -> Vec<String> {
        self.args
            .iter()
            .filter_map(|el| {
                if let Element::VariableName(el, _) = el {
                    Some(el.get_name())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
    }
    pub fn execute_block(
        &self,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            self.block
                .execute(ExecuteContext::unbound(cx, sc).token(token))
                .await
        })
    }
}
