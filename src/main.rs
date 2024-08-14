mod command;
mod compilation;
mod config;
mod fs;
mod judge;
mod printer;
mod process;
mod task;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use color_eyre::eyre::Result;

use crate::command::{
    build::{build, BuildArgs},
    completion::{print_completion, CompletionArgs},
    expand::{expand, ExpandArgs},
    test::{test, TestArgs},
};

/// コマンドライン引数
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(alias = "e")]
    /// ソースコード中の#includeを展開する
    Expand(ExpandArgs),
    #[command(alias = "b")]
    /// ソースコードをビルドする
    Build(BuildArgs),
    #[command(alias = "t")]
    /// ソースコードをテストする
    Test(TestArgs),
    /// シェル補完関数を生成する
    Completion(CompletionArgs),
}

fn main() -> Result<()> {
    let args = Cli::parse();

    config::init(args.verbose.log_level_filter())?;
    match args.command {
        Commands::Expand(args) => {
            expand(&args)?;
        }
        Commands::Build(args) => {
            build(&args)?;
        }
        Commands::Completion(args) => {
            print_completion(args.shell);
        }
        Commands::Test(args) => {
            test(&args)?;
        }
    }
    Ok(())
}
