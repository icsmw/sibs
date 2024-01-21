mod args;
pub mod error;

use crate::{
    inf::{
        context::Context,
        operator::Operator,
        scenario::Scenario,
        term::{Display, Term},
        tracker::Tracker,
    },
    reader::{self, entry::Component},
};
use args::Arguments;
use error::E;
use std::env::{self, current_dir};

use self::args::Argument;

pub async fn read(tracker: &Tracker) -> Result<Option<Context>, E> {
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
    let mut term = Term::new();
    let mut defaults = Arguments::new(&mut income)?;
    if defaults.has::<args::version::Version>() {
        run::<args::version::Version>(&[], &mut defaults, &mut Context::unbound())?;
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
    let mut cx = Context::new(term, tracker.clone(), scenario);
    let components = reader::read_file(&mut cx)?;
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
        if let Some(component) = components.iter().find(|comp| comp.name == component) {
            component.process(&components, &income, &mut cx).await?;
            Ok(Some(cx))
        } else {
            Err(E::ComponentNotExists(component.to_string()))
        }
    } else {
        Err(E::NoArguments)
    }
}
