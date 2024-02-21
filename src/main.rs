use std::process::exit;

mod cli;
mod entry;
mod error;
mod executors;
mod inf;
mod reader;

use inf::{context::Context, tracker::Tracker};

fn main() {
    async_io::block_on(async {
        let cfg = cli::get_tracker_configuration();
        match cfg {
            Ok(cfg) => {
                let mut cx = match Context::with_tracker(Tracker::new(cfg)) {
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
                    cx.map.borrow().post_reports();
                    exit(1);
                }
            }
            Err(err) => {
                eprintln!("{err}");
                exit(1);
            }
        }
    });
}
