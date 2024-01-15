mod args;
pub mod error;
pub mod location;

use crate::{
    inf::{
        reporter::{Display, Reporter},
        runner::Runner,
    },
    reader::{self, entry::Component},
};
use args::Arguments;
use error::E;
use location::Location;
use std::env;

use self::args::Argument;

pub fn read() -> Result<(), E> {
    fn run<T: Argument<T> + 'static>(
        components: &[Component],
        arguments: &mut Arguments,
        reporter: &mut Reporter,
        location: &Location,
    ) -> Result<(), E> {
        if let Some(arg) = arguments.get_mut::<T>() {
            arg.action(components, reporter, location)
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
        run::<args::version::Version>(&[], &mut defaults, &mut reporter, &Location::dummy())?;
        if !income.is_empty() {
            reporter.err(format!("Ingore next arguments: {}", income.join(", ")));
        }
        return Ok(());
    }
    let location = if let Some(target) = defaults.get::<args::target::Target>() {
        Location::from(target.get())?
    } else {
        match Location::new() {
            Ok(location) => location,
            Err(_) => {
                reporter.print("Scenario file hasn't been found.\n\n");
                reporter.bold("OPTIONS\n");
                reporter.step_right();
                defaults.display(&mut reporter);
                return Ok(());
            }
        }
    };
    let components = reader::read_file(&location.filename)?;
    let no_actions = defaults.has::<args::help::Help>();
    run::<args::help::Help>(&components, &mut defaults, &mut reporter, &location)?;
    if no_actions {
        return Ok(());
    }
    if let Some(component) = if components.is_empty() {
        None
    } else {
        Some(income.remove(0))
    } {
        if let Some(component) = components.iter().find(|comp| comp.name == component) {
            component.run(&components, income, &mut reporter)?;
            Ok(())
        } else {
            Err(E::ComponentNotExists(component.to_string()))
        }
    } else {
        Err(E::NoArguments)
    }
}
