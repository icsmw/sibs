mod ast;
mod executor;
mod utils;

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use parser::*;

pub(crate) use asttree::*;
pub(crate) use boxed::boxed;
pub(crate) use diagnostics::*;
use lexer::SrcLink;
pub(crate) use lexer::{Keyword, Kind};
pub(crate) use runtime::error::E;
pub(crate) use runtime::*;
pub(crate) use semantic::*;
pub use executor::*;
pub use utils::*;
use uuid::Uuid;

pub trait Interpret {
    fn interpret(&self, _env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>>;
}

pub trait Execute
where
    Self: Sync,
{
    fn uuid(&self) -> &Uuid;
    fn block(&self) -> &LinkedNode;
    fn link(&self) -> SrcLink;
    #[boxed]
    fn before(&self, _env: InterpreterEnvironment) -> GtPinnedResult<'_, LinkedErr<E>> {
        Ok(true)
    }
    #[boxed]
    fn exec(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let before = self.before(env.clone()).await?;
        if !before {
            return Ok(RtValue::Skipped);
        }
        let InterpreterEnvironment { cx, .. } = env.clone();
        cx.returns()
            .open_cx(self.uuid())
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.link()).into()))?;
        let mut result = self.block().interpret(env.clone()).await?;
        result = if let Some(result) = cx
            .returns()
            .withdraw_vl(self.uuid())
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.link()).into()))?
        {
            result
        } else {
            result
        };
        cx.returns()
            .close_cx()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.link()).into()))?;
        Ok(result)
    }
}
