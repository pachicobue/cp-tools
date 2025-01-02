mod commands;
mod judge;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use itertools::Itertools;
use thiserror::Error;

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
    let res = inner();
    if res.is_err() {
        let msg = get_trace_messages(&res.unwrap_err());
        log::error!("{}", msg);
    }
}

fn inner() -> Result<(), ApplicationError> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    exec_command(&args.command)?;

    Ok(())
}

fn get_trace_messages(error: &impl std::error::Error) -> String {
    let mut messages = vec![];
    get_trace_messages_inner(error, &mut messages);
    let children_message = &messages
        .iter()
        .map(|message| format!("* {message}"))
        .collect_vec()
        .join("\n");
    format!("{}\n{children_message}", error)
}

fn get_trace_messages_inner(error: &dyn std::error::Error, messages: &mut Vec<String>) {
    messages.push(error.to_string());
    if let Some(source) = error.source() {
        get_trace_messages_inner(source, messages);
    }
}
