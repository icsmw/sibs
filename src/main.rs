use std::process::exit;

mod cli;
mod functions;
mod inf;
mod reader;

fn main() {
    async_io::block_on(async {
        if let Err(err) = cli::read().await {
            eprint!("Error: {err}");
            exit(1);
        }
    });
}
