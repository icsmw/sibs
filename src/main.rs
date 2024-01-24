use std::process::exit;

mod cli;
mod error;
mod executors;
mod inf;
mod reader;

use inf::tracker::Tracker;

fn main() {
    async_io::block_on(async {
        let tracker = Tracker::new();
        let result = cli::read(&tracker).await;
        if tracker.shutdown().await.is_err() {
            eprintln!("Fail to shutdown tracker correctly");
        }
        if let Err(err) = result {
            eprint!("Error: {err}");
            exit(1);
        }
    });
}
