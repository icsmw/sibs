pub mod action;
pub mod arg;
pub mod exertion;

use crate::{
    cli::error::E,
    elements::Component,
    inf::{
        term::{self, Term},
        AnyValue, Context,
    },
};
pub use action::*;
pub use arg::*;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Arguments {
    pub actions: HashMap<String, Box<dyn Action>>,
}

impl Arguments {
    pub fn new(args: &mut Vec<String>) -> Result<Self, E> {
        let mut actions: HashMap<String, Box<dyn Action>> = HashMap::new();
        for action in vec![
            exertion::Version::read(args)?,
            exertion::Scenario::read(args)?,
            exertion::Format::read(args)?,
            exertion::Help::read(args)?,
            exertion::Output::read(args)?,
            exertion::LogFile::read(args)?,
            exertion::Trace::read(args)?,
        ]
        .into_iter()
        .flatten()
        {
            let key = action.key();
            if actions.insert(action.key(), action).is_some() {
                Err(E::DuplicateOfKey(key))?;
            }
        }
        Ok(Self { actions })
    }
    pub async fn run<T: Argument + 'static>(
        &self,
        components: &[Component],
        cx: &mut Context,
    ) -> Result<Option<AnyValue>, E> {
        if let Some(action) = self.actions.get(&T::key()) {
            Ok(Some(action.action(components, cx).await?))
        } else {
            Ok(None)
        }
    }
    pub async fn run_no_cx<T: Argument + 'static>(&self) -> Result<Option<AnyValue>, E> {
        if let Some(action) = self.actions.get(&T::key()) {
            Ok(Some(
                action
                    .action(&[], &mut Context::create().unbound()?)
                    .await?,
            ))
        } else {
            Ok(None)
        }
    }
    pub async fn get_value_no_cx<T: Argument + 'static, O: Clone + 'static>(
        &self,
    ) -> Result<Option<O>, E> {
        if let Some(action) = self.actions.get(&T::key()) {
            Ok(action
                .action(&[], &mut Context::create().unbound()?)
                .await?
                .get_as::<O>()
                .cloned())
        } else {
            Ok(None)
        }
    }
    pub async fn all_without_context(&self) -> Result<bool, E> {
        let actions = self
            .actions
            .iter()
            .filter(|(_, a)| a.no_context())
            .map(|(_, a)| a)
            .collect::<Vec<&Box<dyn Action>>>();
        if actions.is_empty() {
            Ok(false)
        } else if actions.len() != 1 {
            Err(E::NotSupportedMultipleArguments(
                actions
                    .iter()
                    .map(|a| a.key())
                    .collect::<Vec<String>>()
                    .join(", "),
            ))
        } else {
            let _ = actions[0]
                .action(&[], &mut Context::create().unbound()?)
                .await?;
            Ok(true)
        }
    }
    pub fn has<T: Argument + 'static>(&self) -> bool {
        self.actions.contains_key(&T::key())
    }
}

impl term::Display for Arguments {
    fn display(&self, term: &mut Term) {
        term.print_fmt(
            &[
                exertion::Scenario::desc(),
                exertion::Help::desc(),
                exertion::Trace::desc(),
                exertion::Output::desc(),
                exertion::LogFile::desc(),
                exertion::Format::desc(),
                exertion::Version::desc(),
            ]
            .iter()
            .flat_map(|desc| {
                [
                    vec![format!("{}>>{}", desc.key.join(", "), desc.desc)],
                    desc.pairs
                        .iter()
                        .map(|(key, value)| format!("{}>>{}", key, value))
                        .collect::<Vec<String>>(),
                ]
                .concat()
            })
            .collect::<Vec<String>>(),
        );
    }
}
