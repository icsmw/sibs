mod args;
mod error;
mod location;
mod reporter;

use args::Argument;
use error::E;
use location::Location;
use std::{any::Any, env};

type AnyArgument = Box<dyn Argument<dyn Any>>;

pub fn read() -> Result<(), E> {
    let mut args = env::args().collect::<Vec<String>>();
    let location = if args.is_empty() {
        Location::new()?
    } else {
        args.remove(0);
        if let Some(target) = args::target::Target::read(&mut args)? {
            Location::from(target.get())?
        } else {
            Location::new()?
        }
    };
    // let mut arguments: Vec<AnyArgument> = vec![];
    // let a: &dyn Fn(&mut Vec<String>) -> Result<Option<dyn Any>, E> = &args::help::Help::read;
    // let defaults: Vec<Fn(&mut Vec<String>) -> Result<Option<dyn Any>, E>> =
    //     vec![args::help::Help::read, args::target::Target::read];
    while !args.is_empty() {}
    reporter::associated("scenario", location.filename.to_str().unwrap());
    Ok(())
}
