mod args;
mod error;
pub mod location;
pub mod reporter;

use crate::reader::{self, entry::Component};
use args::Arguments;
use error::E;
use location::Location;
use std::env;

use self::{
    args::Argument,
    reporter::{Display, Reporter},
};

pub fn read() -> Result<(), E> {
    fn run<T: Argument<T> + 'static>(
        components: Vec<Component>,
        arguments: &mut Arguments,
        reporter: &mut Reporter,
        location: &Location,
    ) -> Result<(), E> {
        if let Some(arg) = arguments.get_mut::<T>() {
            arg.action(&components, reporter, location)
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
    println!("{location:?}");
    let components = reader::read_file(&location.filename)?;
    run::<args::help::Help>(components, &mut defaults, &mut reporter, &location)?;
    Ok(())
}
