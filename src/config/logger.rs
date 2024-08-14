use color_eyre::eyre::Result;
use log::LevelFilter;

use crate::config::metadata::crate_name;

pub fn init(level: LevelFilter) -> Result<()> {
    colog::default_builder()
        .filter_module(&crate_name(), level)
        .try_init()?;
    Ok(())
}
