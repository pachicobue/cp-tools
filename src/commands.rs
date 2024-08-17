pub mod build;
pub mod compilation;
pub mod compile_opts;
pub mod completion;
pub mod expand;
pub mod test;

use std::fmt;

use clap::Subcommand;
use color_eyre::eyre::Result;

use crate::commands::{
    build::{build, BuildArgs},
    completion::{print_completion, CompletionArgs},
    expand::{expand, ExpandArgs},
    test::{test, TestArgs},
};

#[derive(Subcommand, Debug)]
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
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Command::Expand(_) => "Expand",
            Command::Build(_) => "Build",
            Command::Test(_) => "Test",
            Command::Completion(_) => "Completion",
        };
        str.fmt(f)
    }
}

pub(crate) fn exec_command(command: &Command) -> Result<()> {
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
