use std::env;

pub(crate) const fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
