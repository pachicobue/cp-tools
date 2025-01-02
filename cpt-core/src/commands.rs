pub mod test;

use clap::Subcommand;
use thiserror::Error;

use crate::commands::test::{test, TestArgs, TestCommandError};

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    #[command(visible_alias = "t")]
    Test(TestArgs),
}

#[derive(Error, Debug)]
pub(crate) enum CommandError {
    #[error("Test command failed")]
    TestFailed(#[from] TestCommandError),
}

pub(crate) fn exec_command(command: &Command) -> Result<(), CommandError> {
    match command {
        Command::Test(args) => {
            test(args)?;
        }
    }
    Ok(())
}
