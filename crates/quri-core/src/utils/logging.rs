use anyhow::{Context, Result};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

/// Initializes Quri's rolling application log.
pub fn setup_logging() -> Result<tracing_appender::non_blocking::WorkerGuard> {
    let state_dir = dirs::state_dir()
        .or_else(dirs::data_local_dir)
        .or_else(dirs::data_dir)
        .context("cannot determine the platform application data directory")?;

    let log_dir = state_dir.join("quri").join("logs");
    std::fs::create_dir_all(&log_dir)
        .with_context(|| format!("failed to create log directory: {}", log_dir.display()))?;

    // Creates files such as quri.log.2026-09-29 and rotates them daily.
    let file_appender = tracing_appender::rolling::daily(log_dir, "quri.log");
    let (non_blocking_writer, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("quri=info"));
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(false)
        .with_writer(non_blocking_writer)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|error| anyhow::anyhow!("failed to install global logger: {error}"))?;

    Ok(guard)
}
