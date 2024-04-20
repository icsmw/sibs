#![windows_subsystem = "windows"]
use std::process::exit;
mod cli;
mod elements;
mod error;
mod executors;
mod inf;
mod reader;
mod runners;

use inf::journal::Journal;

#[tokio::main]
async fn main() {
    let Ok(cfg) = cli::get_journal_configuration()
        .await
        .map_err(|e| eprintln!("Fail to init journal: {e}"))
    else {
        exit(1);
    };
    let journal = Journal::init(cfg);
    if let Err(err) = cli::process(journal.clone()).await {
        journal.err("cli::process", &err.to_string());
    }
    if let Err(err) = journal.destroy().await {
        eprintln!("{err}");
        exit(1);
    }
}
