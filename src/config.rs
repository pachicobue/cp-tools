pub mod compile_opts;
pub mod dirs;
pub mod logger;
pub mod metadata;
pub mod testcase;
use color_eyre::eyre::Result;
use log::LevelFilter;

pub(crate) fn init(level: LevelFilter) -> Result<()> {
    console_subscriber::init();
    color_eyre::install()?;
    logger::init(level)?;
    dirs::init()?;
    Ok(())
}
