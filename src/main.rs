use std::process::exit;
mod cli;
mod elements;
mod error;
mod executors;
mod inf;
mod reader;

use inf::{context::Context, tracker::Tracker};

#[tokio::main]
async fn main() {
    let cfg = cli::get_tracker_configuration().await;
    match cfg {
        Ok(cfg) => {
            let mut cx = match Context::create().with_tracker(Tracker::new(cfg)) {
                Ok(cx) => cx,
                Err(err) => {
                    eprint!("{err}");
                    exit(1);
                }
            };
            let result = cli::read(&mut cx).await;
            if cx.tracker.shutdown().await.is_err() {
                eprintln!("Fail to shutdown tracker correctly");
            }
            if let Err(err) = result {
                eprintln!("{err}");
                // if let Err(err) = cx.sources.assign_error(&err) {
                //     eprintln!("{err}");
                // }
                // cx.sources.post_reports();
                exit(1);
            }
        }
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    }
}
