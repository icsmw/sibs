#![windows_subsystem = "windows"]
use std::process::exit;
mod cli;
mod elements;
mod error;
mod functions;
mod inf;
mod reader;
mod runners;

use inf::journal::Journal;

#[allow(clippy::needless_return)]
#[tokio::main]
async fn main() {
    let Ok(cfg) = cli::get_journal_configuration()
        .await
        .map_err(|e| eprintln!("Fail to init journal: {e}"))
    else {
        exit(1);
    };
    let journal = Journal::unwrapped(cfg);
    let code = match cli::process(journal.clone()).await {
        Ok(code) => code.code(),
        Err(err) => {
            journal.err("cli::process", err.to_string());
            1
        }
    };
    if let Err(err) = journal.destroy().await {
        eprintln!("{err}");
        exit(1);
    } else {
        exit(code);
    };
}
