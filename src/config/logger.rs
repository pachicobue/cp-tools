use ::{
    anyhow::Result,
    fern::colors::{Color, ColoredLevelConfig},
};

pub(crate) fn init() -> Result<()> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Yellow)
        .debug(Color::Cyan)
        .trace(Color::White);
    Ok(fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{col_begin}{attr_begin}[{level}][{target}]{end} {col_begin}{message}{end}",
                col_begin = format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                attr_begin = (if record.level() <= log::Level::Debug {
                    "\x1B[1m"
                } else {
                    ""
                }),
                target = record.target(),
                level = record.level(),
                message = message,
                end = (if record.level() <= log::Level::Debug {
                    "\x1B[0m"
                } else {
                    ""
                })
            ))
        })
        .level(log::LevelFilter::Warn)
        .level_for("cp_tools", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()?)
}
