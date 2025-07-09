use colored::*;
use env_logger::WriteStyle;
use log::LevelFilter;
use std::io::Write as _;

pub use colored;
pub use env_logger;
pub use log;

/// Creates a new `env_logger::Builder` instance with better defaults for logging.
/// targets: `RUST_LOG=[target][=][level][,...]`
pub fn build(log_lvl: Option<LevelFilter>) -> env_logger::Builder {
    let mut builder = env_logger::builder();

    builder.parse_env("RUST_LOG");

    if let Some(log_lvl) = log_lvl {
        builder.filter(None, log_lvl);
    }

    builder
        .format(|buf, record| {
            let style = buf.default_level_style(record.level());
            let level_style = format!("{style}{}{style:#}", record.level());

            let target_pretty = record
                .target()
                .split("::")
                .next()
                .unwrap_or_else(|| record.target());

            if cfg!(debug_assertions) {
                writeln!(
                    buf,
                    "[{}] [{}]: {}",
                    level_style,
                    target_pretty,
                    record.args()
                )
            } else {
                let timestamp = chrono::Local::now()
                    .format("%I:%M:%S%p")
                    .to_string()
                    .yellow()
                    .dimmed();

                writeln!(
                    buf,
                    "[{}] [{}] [{}]: {}",
                    timestamp,
                    level_style,
                    target_pretty,
                    record.args()
                )
            }
        })
        .format_level(true)
        .write_style(WriteStyle::Always);

    builder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        build(Some(LevelFilter::Error)).init();
        log::trace!("This is a test log");
        log::debug!("This is a test log");
        log::info!("This is a test log");
        log::warn!("This is a test log");
        log::error!("This is a test log");
    }
}
