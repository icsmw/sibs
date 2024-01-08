mod args;
mod error;
mod location;
pub mod reporter;

use crate::reader;
use args::Arguments;
use error::E;
use location::Location;
use std::env;

use self::reporter::{Display, Reporter};

pub fn read() -> Result<(), E> {
    let mut income = env::args().collect::<Vec<String>>();
    if !income.is_empty() {
        income.remove(0);
    }
    let defaults = Arguments::new(&mut income)?;
    let location = if let Some(target) = defaults.get::<args::target::Target>() {
        Location::from(target.get())?
    } else {
        Location::new()?
    };
    let components = reader::read_file(&location.filename)?;
    let mut reporter = Reporter::new();
    println!("{defaults:?}");
    if let Some(help) = defaults.get::<args::help::Help>() {
        if help.context().is_none() && components.is_empty() {
            defaults.display(&mut reporter);
        } else {
            reporter.bold("SCENARIO:\n");
            reporter.step_right();
            reporter.print(&format!(
                "{}{}\n\n",
                reporter.offset(),
                location.filename.to_str().unwrap()
            ));
            reporter.step_left();
            let with_context = components
                .iter()
                .filter(|comp| comp.cwd.is_some())
                .map(|comp| {
                    (
                        comp.name.clone(),
                        comp.meta
                            .as_ref()
                            .map(|meta| meta.as_string())
                            .unwrap_or_default(),
                    )
                })
                .collect::<Vec<(String, String)>>();
            if !with_context.is_empty() {
                reporter.bold("COMPONENTS:\n");
                reporter.step_right();
                reporter.pairs(with_context);
                reporter.step_left();
            }
            if components.iter().any(|comp| comp.cwd.is_none()) {
                reporter.bold("\nCOMMANDS:\n");
            }
            reporter.step_right();
            components
                .iter()
                .filter(|comp| comp.cwd.is_none())
                .for_each(|comp| {
                    comp.tasks.iter().filter(|t| t.has_meta()).for_each(|task| {
                        task.display(&mut reporter);
                    });
                });
            reporter.step_left();
        }
    }
    Ok(())
}
