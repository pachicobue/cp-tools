use {color_eyre::eyre::Result, log::LevelFilter};

pub fn init(level: LevelFilter) -> Result<()> {
    colog::default_builder()
        .filter_module("cp_tools", level)
        .try_init()?;
    Ok(())
}
