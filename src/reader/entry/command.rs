use crate::{
    cli,
    inf::{
        context::Context,
        location::Location,
        reporter::{self, Reporter},
        runner::{self, Runner},
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
        Self { command, token }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.command)
    }
}

impl reporter::Display for Command {
    fn display(&self, reporter: &mut Reporter) {
        reporter.printnl(&self.command);
    }
}

impl Runner for Command {
    fn run(
        &self,
        components: &[Component],
        args: &[String],
        context: &mut Context,
    ) -> Result<runner::Return, cli::error::E> {
        Ok(None)
    }
}
