pub mod batch;
pub mod reactive;
pub mod runtwice;
pub mod special;

use clap::{Args, Subcommand};
use color_eyre::eyre::Result;

use self::batch::{batch, BatchArgs};
use crate::core::judge::Verdict;

#[derive(Args, Debug)]
pub(crate) struct TestArgs {
    /// テストコマンド
    #[command(subcommand)]
    command: TestCommands,
}

#[derive(Subcommand, Debug)]
enum TestCommands {
    /// Batch Judge
    #[command(alias = "b")]
    Batch(BatchArgs),
    // /// Special Judge
    // #[command(alias = "s")]
    // Special(SpecialArgs),
    // /// Reactive Judge
    // #[command(aliases=["ra", "i"])]
    // Reactive(ReactiveArgs),
    // /// RunTwice Judge
    // #[command(alias = "rt")]
    // Runtwice(RuntwiceArgs),
}

pub(crate) fn test(args: &TestArgs) -> Result<Vec<Verdict>> {
    match &args.command {
        TestCommands::Batch(args) => batch(&args),
        // TestCommands::Special(args) => special(&args)?,
        // TestCommands::Reactive(args) => reactive(&args)?,
        // TestCommands::Runtwice(args) => runtwice(&args)?,
    }
}
