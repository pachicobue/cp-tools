mod commands;
mod judge;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    #[command(subcommand)]
    command: crate::commands::Command,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum ApplicationError {
    #[error("Command Failed")]
    CommandFailed(#[from] crate::commands::Error),
}

fn main() {
    use cpt_stdx::error::stacktrace;
    if let Err(e) = main_inner() {
        log::error!("Error occurred.\n{}", stacktrace(e));
    }
}

fn main_inner() -> Result<(), ApplicationError> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    args.command.run()?;
    Ok(())
}
