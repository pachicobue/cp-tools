pub mod batch_test;

use clap::Subcommand;
use thiserror::Error;

use crate::commands::batch_test::{test, BatchTestArgs, BatchTestCommandError};

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    #[command(visible_alias = "t")]
    Test(BatchTestArgs),
}

#[derive(Error, Debug)]
pub(crate) enum CommandError {
    #[error("Test command failed")]
    TestFailed(#[from] BatchTestCommandError),
}

pub(crate) fn exec_command(command: &Command) -> Result<(), CommandError> {
    match command {
        Command::Test(args) => {
            test(args)?;
        }
    }
    Ok(())
}
