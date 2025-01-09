mod commands;
mod judge;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use thiserror::Error;

use cpt_stdx::trace_error::error_trace;

use crate::commands::{exec_command, Command, CommandError};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Error, Debug)]
pub(crate) enum ApplicationError {
    #[error("CommandError")]
    CommandError(#[from] CommandError),
}

fn main() {
    let res = main_inner();
    if res.is_err() {
        let msg = error_trace(&res.unwrap_err());
        log::error!("{}", msg);
    }
}

fn main_inner() -> Result<(), ApplicationError> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    exec_command(&args.command)?;
    Ok(())
}
