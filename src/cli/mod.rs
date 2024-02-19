mod args;
pub mod error;

use crate::{
    entry::Component,
    inf::{
        context::Context,
        operator::Operator,
        scenario::Scenario,
        term::{Display, Term},
        tracker::{self, Tracker},
    },
    reader,
};
use args::Arguments;
use error::E;
use std::{
    env::{self, current_dir},
    path::PathBuf,
};

use self::args::Argument;

fn get_arguments() -> Result<(Vec<String>, Arguments), E> {
    let mut income = env::args().collect::<Vec<String>>();
    if !income.is_empty() {
        income.remove(0);
    }
    let args = Arguments::new(&mut income)?;
    Ok((income, args))
}

pub fn get_tracker_configuration() -> Result<tracker::Configuration, E> {
    let (_, arguments) = get_arguments()?;
    Ok(tracker::Configuration {
        output: arguments
            .get::<args::output::Output>()
            .map(|arg| arg.output.clone())
            .unwrap_or(tracker::Output::Progress),
        log_file: arguments
            .get::<args::log_file::LogFile>()
            .map(|arg| PathBuf::from(arg.file.to_owned())),
        trace: arguments
            .get::<args::trace::Trace>()
            .map(|arg| arg.state)
            .unwrap_or(false),
    })
}

pub async fn read(tracker: &Tracker) -> Result<Option<Context>, E> {
    fn run<T: Argument<T> + 'static>(
        components: &[Component],
        arguments: &mut Arguments,
        cx: &mut Context,
    ) -> Result<(), E> {
        arguments
            .get_mut::<T>()
            .map_or(Ok(()), |arg| arg.action(components, cx))
    }
    let mut term = Term::new();
    let (mut income, mut defaults) = get_arguments()?;
    if defaults.has::<args::version::Version>() {
        run::<args::version::Version>(&[], &mut defaults, &mut Context::unbound()?)?;
        if !income.is_empty() {
            term.err(format!("Ingore next arguments: {}", income.join(", ")));
        }
        return Ok(None);
    }
    let scenario = if let Some(target) = defaults.get::<args::target::Target>() {
        Scenario::from(&current_dir()?.join(target.get()).canonicalize()?)?
    } else {
        match Scenario::new() {
            Ok(scenario) => scenario,
            Err(_) => {
                term.print("Scenario file hasn't been found.\n\n");
                term.bold("OPTIONS\n");
                term.step_right();
                defaults.display(&mut term);
                return Ok(None);
            }
        }
    };
    let mut cx = Context::new(term, tracker.clone(), scenario)?;
    let components = reader::read_file(&mut cx).await?;
    let no_actions = defaults.has::<args::help::Help>() || income.is_empty();
    run::<args::help::Help>(&components, &mut defaults, &mut cx)?;
    if no_actions {
        return Ok(Some(cx));
    }
    if let Some(component) = if components.is_empty() {
        None
    } else {
        Some(income.remove(0))
    } {
        if let Some(component) = components
            .iter()
            .find(|comp| comp.name.to_string() == component)
        {
            component
                .process(Some(component), &components, &income, &mut cx)
                .await?;
            Ok(Some(cx))
        } else {
            Err(E::ComponentNotExists(component.to_string()))
        }
    } else {
        Err(E::NoArguments)
    }
}
