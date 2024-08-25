pub mod build;
pub mod completion;
pub mod expand;
pub mod test;

use clap::Subcommand;
use strum;

use crate::{
    commands::{
        build::{build, BuildArgs},
        completion::{print_completion, CompletionArgs},
        expand::{expand, ExpandArgs},
        test::{test, TestArgs},
    },
    core::error::CommandError,
};

#[derive(Subcommand, Debug, strum::Display)]
pub(crate) enum Command {
    #[command(visible_alias = "e")]
    /// ソースコード中の#includeを展開する
    Expand(ExpandArgs),
    #[command(visible_alias = "b")]
    /// ソースコードをビルドする
    Build(BuildArgs),
    #[command(visible_alias = "t")]
    /// ソースコードをテストする(Batch)
    Test(TestArgs),
    /// シェル補完関数を生成する
    Completion(CompletionArgs),
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
