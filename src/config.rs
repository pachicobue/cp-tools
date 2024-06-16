pub mod compile_opts;
pub mod dirs;
pub mod logger;
use ::anyhow::Result;

pub(crate) fn init() -> Result<()> {
    logger::init()?;
    dirs::init()?;
    compile_opts::init()?;
    Ok(())
}
