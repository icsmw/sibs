use std::process::exit;

mod cli;
mod entry;
mod error;
mod executors;
mod inf;
mod reader;

use inf::tracker::Tracker;

fn main() {
    async_io::block_on(async {
        let cfg = cli::get_tracker_configuration();
        match cfg {
            Ok(cfg) => {
                let tracker = Tracker::new(cfg);
                let result = cli::read(&tracker).await;
                if tracker.shutdown().await.is_err() {
                    eprintln!("Fail to shutdown tracker correctly");
                }
                if let Err(err) = result {
                    eprint!("Error: {err}");
                    exit(1);
                }
            }
            Err(err) => {
                panic!("{err}");
            }
        }
    });
}
