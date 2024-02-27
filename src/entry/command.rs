use crate::{
    entry::Component,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
        spawner,
        term::{self, Term},
        tracker::Logs,
    },
    reader::{chars, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Command {
    pub command: String,
    pub token: usize,
}

impl Command {
    pub fn new(command: String, token: usize) -> Result<Self, E> {
        let mut command = command.trim().to_owned();
        if command.ends_with(chars::SEMICOLON) {
            let _ = command.pop();
        }
        if command.is_empty() {
            Err(E::EmptyCommand)
        } else {
            Ok(Self { command, token })
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.command)
    }
}

impl term::Display for Command {
    fn display(&self, term: &mut Term) {
        term.printnl(&self.command);
    }
}

impl Operator for Command {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        _: Option<&'a Component>,
        _: &'a [Component],
        _: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async {
            let cwd = cx.cwd.as_ref().ok_or(operator::E::NoCurrentWorkingFolder)?;
            let job = cx
                .tracker
                .create_job(
                    &format!("{}: {}", cx.scenario.to_relative_path(cwd), self.command),
                    None,
                )
                .await?;
            match spawner::run(&self.command, cwd, &job).await {
                Ok(status) => {
                    if status.success() {
                        job.success();
                        Ok(Some(AnyValue::new(())))
                    } else {
                        job.fail();
                        Err(operator::E::SpawnedProcessExitWithError)
                    }
                }
                Err(e) => {
                    job.err(&e.to_string());
                    job.fail();
                    Err(e)?
                }
            }
        })
    }
}

#[cfg(test)]
mod proptest {

    use crate::{entry::command::Command, inf::tests::*};
    use proptest::prelude::*;

    impl Arbitrary for Command {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Command);
            let boxed = Just(Command::new("ls".to_owned(), 0).unwrap()).boxed();
            scope.write().unwrap().exclude(Entity::Command);
            boxed
        }
    }
}
