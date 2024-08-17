mod commands;
mod config;
mod core;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use color_eyre::eyre::Result;

use crate::commands::{exec_command, Command};

/// コマンドライン引数
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    config::init(args.verbose.log_level_filter())?;
    exec_command(&args.command)?;
    Ok(())
}
