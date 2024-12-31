use log::LevelFilter;
pub(crate) fn init(level_filter: LevelFilter) {
    env_logger::Builder::new().filter_level(level_filter).init();
}
