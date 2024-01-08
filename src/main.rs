mod cli;
mod context;
mod functions;
mod reader;

fn main() {
    if let Err(err) = cli::read() {
        eprintln!("{err}");
    }
}
