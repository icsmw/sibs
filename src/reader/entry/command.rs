use crate::{
    cli,
    inf::{
        context::Context,
        runner::{self, Runner},
        spawner,
        term::{self, Term},
    },
    reader::entry::Component,
};
use std::fmt;

#[derive(Debug)]
pub struct Command {
    pub command: String,
    pub token: usize,
}

impl Command {
    pub fn new(command: String, token: usize) -> Self {
        Self {
            command: command.trim().to_owned(),
            token,
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

impl Runner for Command {
    async fn run(
        &self,
        components: &[Component],
        args: &[String],
        cx: &mut Context,
    ) -> Result<runner::Return, cli::error::E> {
        let cwd = cx
            .cwd
            .as_ref()
            .ok_or(cli::error::E::NoCurrentWorkingFolder)?;
        let res = spawner::spawn(&self.command, cx).await;
        println!(">>>>>>>>>>>>>>>>>>> wiuuu: {cx:?}");
        println!(">>>>>>>>>>>>>>>>>>> wiuuu: {res:?}");
        Ok(None)
    }
}
