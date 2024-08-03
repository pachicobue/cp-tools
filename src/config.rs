pub mod compile_opts;
pub mod dirs;
pub mod logger;
use ::{color_eyre::eyre::Result, log::LevelFilter};

pub(crate) fn init(level: LevelFilter) -> Result<()> {
    color_eyre::install()?;
    logger::init(level)?;
    dirs::init()?;
    compile_opts::init()?;
    Ok(())
}
