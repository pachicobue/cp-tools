use color_eyre::eyre::Result;
use log::LevelFilter;

use crate::config::metadata::CRATE_NAME;

pub fn init(level: LevelFilter) -> Result<()> {
    colog::default_builder()
        .filter_module(CRATE_NAME, level)
        .try_init()?;
    Ok(())
}
