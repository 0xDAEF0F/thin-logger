use std::io::Write as _;

pub use colored::*;
pub use env_logger::*;
pub use log::*;

/// Creates a new `env_logger::Builder` instance with better defaults for
/// logging.
pub fn new_builder(log_lvl: LevelFilter) -> Builder {
    let mut builder = env_logger::builder();

    builder
        .format(|buf, record| {
            let timestamp = chrono::Local::now()
                .format("%H:%M:%S%p")
                .to_string()
                .yellow()
                .dimmed();

            let style = buf.default_level_style(record.level());
            let level_style = format!("{style}{}{style:#}", record.level());

            let target_pretty =
                record.target().split("::").next().unwrap_or_else(|| record.target());

            writeln!(
                buf,
                "[{}] [{}] [{}]: {}",
                timestamp,
                level_style,
                target_pretty,
                record.args()
            )
        })
        .format_level(true)
        .write_style(WriteStyle::Always)
        .filter(None, log_lvl);

    builder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_logs() {
        new_builder(LevelFilter::Trace).init();
        log::trace!("This is a test log");
        log::debug!("This is a test log");
        log::info!("This is a test log");
        log::warn!("This is a test log");
        log::error!("This is a test log");
    }
}
