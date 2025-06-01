use std::{env, fs, io, path::Path};
use tracing::debug;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, EnvFilter};

const LOG_DIR: &str = "logs";

pub fn init() -> io::Result<WorkerGuard> {
    let path = env::temp_dir().join(LOG_DIR);
    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }

    let file_appender = tracing_appender::rolling::never(&path, "sibs.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    fmt()
        .with_writer(non_blocking)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();
    debug!("Log is inited");
    Ok(guard)
}
