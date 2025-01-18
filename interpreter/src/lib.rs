mod ast;
mod utils;

#[cfg(test)]
mod tests;

pub(crate) use asttree::*;
pub(crate) use boxed::boxed;
pub(crate) use diagnostics::*;
use lexer::SrcLink;
pub(crate) use lexer::{Keyword, Kind};
pub(crate) use runtime::error::E;
pub(crate) use runtime::*;
pub(crate) use utils::*;

#[cfg(test)]
pub(crate) use parser::*;
#[cfg(test)]
pub(crate) use semantic::*;
use uuid::Uuid;

pub trait Interpret {
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>>;
}

pub trait Execute
where
    Self: Sync,
{
    fn uuid(&self) -> &Uuid;
    fn block(&self) -> &LinkedNode;
    fn link(&self) -> SrcLink;
    #[boxed]
    fn exec(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        rt.evns
            .open_return_cx(self.uuid())
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.link()).into()))?;
        let mut result = self.block().interpret(rt.clone()).await?;
        result = if let Some(result) = rt
            .evns
            .withdraw_return_vl(self.uuid())
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.link()).into()))?
        {
            result
        } else {
            result
        };
        rt.evns
            .close_return_cx()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.link()).into()))?;
        Ok(result)
    }
}
