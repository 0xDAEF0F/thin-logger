use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use std::io::Write;

pub struct ThinLogger {
    app_level_logs: LevelFilter,
    external_level_logs: LevelFilter,
}

impl ThinLogger {
    pub fn new(app_level_logs: LevelFilter) -> Self {
        ThinLogger {
            app_level_logs,
            external_level_logs: LevelFilter::Off,
        }
    }

    pub fn external_logs(mut self, level: LevelFilter) -> Self {
        self.external_level_logs = level;
        self
    }

    pub fn init(self) -> Result<()> {
        let mut builder = Builder::new();

        builder
            .format(|buf, record| {
                let timestamp = Local::now()
                    .format("%H:%M:%S%p")
                    .to_string()
                    .yellow()
                    .dimmed();
                let style = buf.default_level_style(record.level());
                let level_style = format!("{style}{}{style:#}", record.level());

                let target_pretty = record.target().split("::").next().unwrap();

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
            .write_style(WriteStyle::Always);

        if self.external_level_logs != LevelFilter::Off {
            builder.filter(None, self.external_level_logs);
        } else {
            let crate_mod = env!("CARGO_PKG_NAME").replace('-', "_");
            builder
                .filter(None, LevelFilter::Off)
                .filter_module(&crate_mod, self.app_level_logs);
        }

        builder.try_init().context("Failed to initialize logger")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_logs() -> Result<()> {
        ThinLogger::new(LevelFilter::Trace).init()?;
        log::trace!("This is a test log");
        log::debug!("This is a test log");
        log::info!("This is a test log");
        log::warn!("This is a test log");
        log::error!("This is a test log");
        Ok(())
    }
}
