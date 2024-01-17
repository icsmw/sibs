use std::{env, process::exit};

mod cli;
mod functions;
mod inf;
mod reader;

const SIBS_PRINT_ERRORS: &str = "SIBS_PRINT_ERRORS";

fn main() {
    if let Err(err) = cli::read() {
        if env::vars().any(|(name, value)| {
            name == SIBS_PRINT_ERRORS && (value == "true" || value == "on" || value == "1")
        }) {
            eprint!("Error: {err}");
        }
        exit(1);
    }
}
