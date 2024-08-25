use log::LevelFilter;

use crate::config::metadata::CRATE_NAME;

pub fn init(level: LevelFilter) {
    colog::default_builder()
        .filter_module(CRATE_NAME, level)
        .init();
}
