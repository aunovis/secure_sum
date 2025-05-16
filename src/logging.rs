use flexi_logger::{DeferredNow, Logger, style};
use log::Record;

use crate::Error;

pub(crate) fn init_logging() -> Result<(), Error> {
    let level = log::Level::Info.to_string();
    Logger::try_with_str(level)?
        .log_to_stdout()
        .format(log_format)
        .start()?;
    Ok(())
}

const TS_FORMAT: &str = "%H:%M:%S";

fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "{} [{}] {}",
        style(level).paint(now.format(TS_FORMAT).to_string()),
        style(level).paint(record.level().to_string()),
        style(level).paint(record.args().to_string())
    )
}
