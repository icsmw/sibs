use asttree::*;
use interpreter::{ExecutionOptions, Executor, ExecutorError, InterContext, ScriptOptions};
use runtime::RtValue;
use std::io::{self, Write};

use crate::*;

pub struct Script {
    component: Option<String>,
    task: Option<String>,
    args: Option<Vec<String>>,
    scenario: Scenario,
    ctx: InterContext,
}

impl Script {
    pub fn new(
        scenario: Scenario,
        component: Option<String>,
        task: Option<String>,
        args: Option<Vec<String>>,
    ) -> Result<Self, E> {
        let mut ctx = InterContext::default();
        let preparation =
            interpreter::Script::from_file(&scenario.filepath, ScriptOptions::strict(), &mut ctx);
        if let Some(diagnostics) = ctx.get_diagnostics() {
            let mut dest = io::stderr().lock();
            for err in diagnostics.errors() {
                diagnostics.err(err, &mut dest)?;
                writeln!(dest)?;
            }
        }
        preparation?;
        Ok(Self {
            scenario,
            ctx,
            component,
            task,
            args,
        })
    }

    pub async fn run(&mut self) -> Result<RtValue, E> {
        let component = self.component.take().ok_or(E::ScriptAlreadyExecuted)?;
        let task = self.task.take().ok_or(E::ScriptAlreadyExecuted)?;
        let args = self.args.take().ok_or(E::ScriptAlreadyExecuted)?;
        let options = ExecutionOptions::new(component, task, self.scenario.cwd()?).with_args(args);
        match Executor::new(options).run(&mut self.ctx).await {
            Ok(value) => Ok(value),
            Err(err) => {
                // A shutdown failure may wrap the execution error with its source position.
                let mut execution_err = &err;
                while let ExecutorError::ErrorAndShutdown { err, .. } = execution_err {
                    execution_err = err;
                }
                if let (ExecutorError::Execution(err), Some(diagnostics)) =
                    (execution_err, self.ctx.get_diagnostics())
                {
                    let mut dest = io::stderr().lock();
                    diagnostics.err(err, &mut dest)?;
                    writeln!(dest)?;
                }
                Err(err.into())
            }
        }
    }
    pub fn print(&self) -> Result<(), E> {
        if self.component.is_some() {
            self.print_tasks()
        } else {
            self.print_components()
        }
    }

    fn print_components(&self) -> Result<(), E> {
        let anchor = self.ctx.get_anchor_inner().ok_or(E::NoAnchorNode)?;
        let mut lines = Vec::new();
        anchor
            .get_components_md()
            .iter()
            .for_each(|(component, (md, tasks))| {
                lines.push(format!("- [b]{component}[/b][>>]"));
                lines.extend(md.lines().into_iter().map(|ln| format!("  {ln}")));
                tasks.iter().for_each(|(task, meta)| {
                    lines.push(format!("[>>]- [b]{task}[/b]"));
                    lines.extend(meta.lines().into_iter().map(|ln| format!("[>>]{ln}")));
                });
            });
        term::print(lines.join("\n"));
        Ok(())
    }

    fn print_tasks(&self) -> Result<(), E> {
        let Some(component) = self.component.clone() else {
            return Err(E::NoComponentParameter);
        };

        let anchor = self.ctx.get_anchor_inner().ok_or(E::NoAnchorNode)?;

        let mut lines = vec![format!("[b]{component}[/b]")];
        let Some(component) = anchor.get_component(&component) else {
            return Err(E::ComponentNotFound(component));
        };
        lines.extend(component.get_md().lines());
        if let Node::Root(Root::Component(component)) = component.get_node() {
            component.get_tasks_md().iter().for_each(|(task, meta)| {
                lines.push(format!(" - [b]{task}[/b][>>]"));
                lines.extend(meta.lines().into_iter().map(|ln| format!("[>>]{ln}")));
            });
        }
        term::print(lines.join("\n"));
        Ok(())
    }
}
