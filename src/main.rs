mod commands;
mod config;
mod core;

use core::error::{ApplicationError, ToTraverseErrorMessage};

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

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

/// アプリケーションのエントリポイント
fn main() {
    let res = inner();
    if res.is_err() {
        let msg = res.unwrap_err().to_traverse_error_message();
        log::error!("{}", msg);
    }
}

/// アプリケーションの内部処理 
fn inner() -> Result<(), ApplicationError> {
    let args = Cli::parse();
    config::init(args.verbose.log_level_filter())?;
    exec_command(&args.command)?;
    Ok(())
}
