use flexi_logger::Logger;

use crate::Error;

pub(crate) fn init_logging() -> Result<(), Error> {
    let level = log::Level::Info.to_string();
    Logger::try_with_str(level)?.log_to_stdout().start()?;
    Ok(())
}
