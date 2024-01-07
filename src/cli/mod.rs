mod args;
mod error;
mod location;
mod reporter;

use crate::reader;
use args::Arguments;
use error::E;
use location::Location;
use std::env;

use self::reporter::Description;

pub fn read() -> Result<(), E> {
    let mut income = env::args().collect::<Vec<String>>();
    if !income.is_empty() {
        income.remove(0);
    }
    let mut defaults = Arguments::new(&mut income)?;
    let location = if let Some(target) = defaults.take::<args::target::Target>() {
        Location::from(target.get())?
    } else {
        Location::new()?
    };
    let components = reader::read_file(&location.filename)?;
    println!("{}", components.len());
    if let Some(help) = defaults.take::<args::help::Help>() {
        if help.context().is_none() && components.is_empty() {
            Arguments::desc();
        } else {
            reporter::associated("scenario", location.filename.to_str().unwrap());
            reporter::desc(
                components
                    .iter()
                    .map(|comp| Description {
                        key: vec![comp.name.clone()],
                        desc: "".to_string(),
                    })
                    .collect(),
            );
        }
    }
    Ok(())
}
