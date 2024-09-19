mod api;
pub mod env;
mod error;
pub mod fs;
pub mod globals;
pub mod hash;
pub mod load;
pub mod logs;
pub mod process;
pub mod sc;
pub mod sig;
pub mod str;
pub mod test;
pub mod vec;

use crate::{
    elements::FuncArg,
    error::LinkedErr,
    inf::{Context, Journal, Scope, Store, Value, ValueRef},
};
use api::*;
pub use error::E;

use std::{future::Future, pin::Pin, sync::Arc};
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;

pub type ExecutorPinnedResult = Pin<Box<dyn Future<Output = ExecutorResult> + Send>>;
pub type ExecutorResult = Result<Value, LinkedErr<E>>;
pub type ExecutorFn = fn(Vec<FuncArg>, usize, Context, Scope) -> ExecutorPinnedResult;

#[derive(Debug)]
pub struct ExecutorFnDescription {
    executor: ExecutorFn,
    args: Vec<ValueRef>,
    output: ValueRef,
}

impl ExecutorFnDescription {
    pub fn new(executor: ExecutorFn, args: Vec<ValueRef>, output: ValueRef) -> Self {
        Self {
            executor,
            args,
            output,
        }
    }
    pub fn exec(
        &self,
        args: Vec<FuncArg>,
        token: usize,
        cx: Context,
        sc: Scope,
    ) -> ExecutorPinnedResult {
        let exec = self.executor;
        exec(args, token, cx, sc)
    }
    pub fn output(&self) -> Result<ValueRef, E> {
        Ok(if matches!(self.output, ValueRef::Incoming) {
            let Some(ValueRef::Vec(ty)) = self.args.first() else {
                return Err(E::InvalidIncomeValueType)?;
            };
            ValueRef::Vec(ty.clone())
        } else {
            self.output.clone()
        })
    }
    pub fn args(&self) -> &[ValueRef] {
        &self.args
    }
}
pub trait TryAnyTo<T> {
    fn try_to(&self) -> Result<T, E>;
}

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    str::register(store)?;
    fs::register(store)?;
    env::register(store)?;
    logs::register(store)?;
    globals::register(store)?;
    sc::register(store)?;
    process::register(store)?;
    sig::register(store)?;
    hash::register(store)?;
    test::register(store)?;
    vec::register(store)?;
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Functions {
    tx: UnboundedSender<api::Demand>,
    state: CancellationToken,
}

impl Functions {
    pub fn init(journal: &Journal) -> Result<Self, E> {
        let (tx, mut rx): (UnboundedSender<api::Demand>, UnboundedReceiver<api::Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
        };
        let mut store = Store::<ExecutorFnDescription>::new();
        let journal = journal.owned("Functions", None);
        register(&mut store)?;
        spawn(async move {
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::GetFunctionDescription(name, ty, tx) => {
                        if let Some(desc) = store.get(&name) {
                            if tx.send(Ok(desc)).is_err() {
                                journal.err(format!("Fail send function's execute: {name}"));
                            }
                        } else if let Some(ty) = ty {
                            let mut candidate = None;
                            let mut multiple = false;
                            for (full_name, desc) in store.all().iter() {
                                if let (true, Some(first_arg_ty)) = (
                                    name == full_name.split("::").last().unwrap_or(full_name),
                                    desc.args.first(),
                                ) {
                                    if first_arg_ty.is_compatible(&ty) {
                                        if candidate.is_some() {
                                            multiple = true;
                                            break;
                                        } else {
                                            candidate = Some(desc.clone());
                                        }
                                    }
                                }
                            }
                            if multiple {
                                continue;
                            }
                            if tx
                                .send(if multiple {
                                    Err(E::MultipleFunctionHasBeenFound(name.to_owned()))
                                } else {
                                    candidate.ok_or(E::FunctionNotExists(name))
                                })
                                .is_err()
                            {
                                journal
                                    .err("Fail send response for GetFunctionDescription command");
                            }
                        } else if tx.send(Err(E::FunctionNotExists(name))).is_err() {
                            journal.err("Fail send response for GetFunctionDescription command");
                        }
                    }
                    Demand::Destroy => {
                        break;
                    }
                }
            }
            state.cancel();
        });
        Ok(instance)
    }
    pub async fn destroy(&self) -> Result<(), E> {
        self.tx.send(Demand::Destroy)?;
        self.state.cancelled().await;
        Ok(())
    }
    /// Execute target function
    ///
    /// # Arguments
    ///
    /// * `name` - name of target function
    /// * `args` - arguments
    ///
    /// # Returns
    ///
    /// `Ok(Value)` result of executing
    pub async fn execute(
        &self,
        name: &str,
        args: Vec<FuncArg>,
        args_token: usize,
        ty: Option<Value>,
        cx: Context,
        sc: Scope,
    ) -> Result<Value, LinkedErr<E>> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::GetFunctionDescription(
            name.to_owned(),
            if let Some(v) = ty {
                Some(v.as_ref()?)
            } else {
                None
            },
            tx,
        ))?;
        rx.await??.exec(args, args_token, cx, sc).await
    }
    pub async fn get_func_desc(
        &self,
        name: &str,
        ty: Option<ValueRef>,
    ) -> Result<Arc<ExecutorFnDescription>, LinkedErr<E>> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::GetFunctionDescription(name.to_owned(), ty, tx))?;
        Ok(rx.await??)
    }
}

// TODO: add like proptest for function's verification
// #[cfg(test)]
// mod tests {
//     use crate::{
//         elements::Function, inf::{Configuration, Context, Journal, Scenario}, reader::Sources
//     };

//     #[tokio::test]
//     async fn verifications() {
//         let journal = Journal::unwrapped(Configuration::logs(false));
//         let scenario = Scenario::dummy();
//         let src = Sources::new(&journal);
//         let cx = Context::init(scenario, &src, &journal).expect("Context is created");
//         let all = cx.funcs.get_all().await.expect("Get all functions");
//         all.iter().for_each(|(name, desc)| {
//         });
//     }
// }
