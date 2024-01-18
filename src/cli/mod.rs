mod args;
pub mod error;

use crate::{
    inf::{
        context::Context,
        reporter::{Display, Reporter},
        runner::Runner,
        scenario::Scenario,
        tracker::Tracker,
    },
    reader::{self, entry::Component},
};
use args::Arguments;
use error::E;
use std::{env, path::PathBuf};

use self::args::Argument;

pub fn read() -> Result<(), E> {
    fn run<T: Argument<T> + 'static>(
        components: &[Component],
        arguments: &mut Arguments,
        cx: &mut Context,
    ) -> Result<(), E> {
        if let Some(arg) = arguments.get_mut::<T>() {
            arg.action(components, cx)
        } else {
            Ok(())
        }
    }
    let mut income = env::args().collect::<Vec<String>>();
    if !income.is_empty() {
        income.remove(0);
    }
    let mut reporter = Reporter::new();
    let mut defaults = Arguments::new(&mut income)?;
    if defaults.has::<args::version::Version>() {
        run::<args::version::Version>(&[], &mut defaults, &mut Context::unbound())?;
        if !income.is_empty() {
            reporter.err(format!("Ingore next arguments: {}", income.join(", ")));
        }
        return Ok(());
    }
    let scenario = if let Some(target) = defaults.get::<args::target::Target>() {
        Scenario::from(&target.get())?
    } else {
        match Scenario::new() {
            Ok(scenario) => scenario,
            Err(_) => {
                reporter.print("Scenario file hasn't been found.\n\n");
                reporter.bold("OPTIONS\n");
                reporter.step_right();
                defaults.display(&mut reporter);
                return Ok(());
            }
        }
    };
    let mut cx = Context {
        cwd: PathBuf::new(),
        scenario,
        reporter,
        tracker: Tracker::new(),
    };
    let components = reader::read_file(&mut cx)?;
    let no_actions = defaults.has::<args::help::Help>();
    run::<args::help::Help>(&components, &mut defaults, &mut cx)?;
    if no_actions {
        return Ok(());
    }
    if let Some(component) = if components.is_empty() {
        None
    } else {
        Some(income.remove(0))
    } {
        if let Some(component) = components.iter().find(|comp| comp.name == component) {
            component.run(&components, &income, &mut cx)?;
            Ok(())
        } else {
            Err(E::ComponentNotExists(component.to_string()))
        }
    } else {
        Err(E::NoArguments)
    }
}
