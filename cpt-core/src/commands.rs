pub mod build;
pub mod completion;
pub mod expand;
pub mod test;

use clap::Subcommand;
use strum;
use thiserror::Error;

use crate::commands::{
    build::{build, BuildArgs, BuildCommandError},
    completion::{print_completion, CompletionArgs},
    expand::{expand, ExpandArgs, ExpandCommandError},
    test::{test, TestArgs, TestCommandError},
};

#[derive(Subcommand, Debug, strum::Display)]
pub(crate) enum Command {
    #[command(visible_alias = "e")]
    Expand(ExpandArgs),
    #[command(visible_alias = "b")]
    Build(BuildArgs),
    #[command(visible_alias = "t")]
    Test(TestArgs),
    Completion(CompletionArgs),
}

#[derive(Error, Debug)]
pub(crate) enum CommandError {
    #[error("Test command failed")]
    TestFailed(#[from] TestCommandError),
    #[error("Build command failed")]
    BuildFailed(#[from] BuildCommandError),
    #[error("Expand command failed")]
    ExpandFailed(#[from] ExpandCommandError),
}

pub(crate) fn exec_command(command: &Command) -> Result<(), CommandError> {
    match command {
        Command::Expand(args) => {
            expand(&args)?;
        }
        Command::Build(args) => {
            build(&args)?;
        }
        Command::Test(args) => {
            test(&args)?;
        }
        Command::Completion(args) => {
            print_completion(args.shell);
        }
    }
    Ok(())
}
