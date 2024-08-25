pub mod dirs;
pub mod logger;
pub mod metadata;

use log::LevelFilter;

use crate::core::{error::InitializationError, language};

pub(crate) fn init(level: LevelFilter) -> Result<(), InitializationError> {
    logger::init(level);
    dirs::init();
    language::init()?;
    Ok(())
}
