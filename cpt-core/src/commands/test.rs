pub mod batch;
pub mod reactive;
pub mod special;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Batch Test failed.")]
    Batch(#[from] crate::commands::test::batch::Error),
    #[error("Special Test failed.")]
    Special(#[from] crate::commands::test::special::Error),
    #[error("Reactive Test failed.")]
    Reactive(#[from] crate::commands::test::reactive::Error),
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Command {
    #[command(visible_alias = "b")]
    Batch(crate::commands::test::batch::Args),
    #[command(visible_alias = "s")]
    Special(crate::commands::test::special::Args),
    #[command(visible_alias = "r")]
    Reactive(crate::commands::test::reactive::Args),
}
impl Command {
    pub(crate) fn run(&self) -> Result<(), Error> {
        use crate::commands::test::{batch, reactive, special};
        match self {
            Command::Batch(args) => {
                batch::run(args)?;
            }
            Command::Special(args) => {
                special::run(args)?;
            }
            Command::Reactive(args) => {
                reactive::run(args)?;
            }
        }
        Ok(())
    }
}
