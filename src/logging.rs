use flexi_logger::{DeferredNow, Logger, style};
use log::Record;

use crate::{Arguments, Error};

pub(crate) fn init_logging(args: &Arguments) -> Result<(), Error> {
    let level = if args.quiet {
        log::Level::Error
    } else if args.verbose {
        log::Level::Debug
    } else {
        log::Level::Info
    };
    Logger::try_with_str(level.to_string())?
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
