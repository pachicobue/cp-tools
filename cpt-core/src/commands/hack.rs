pub mod batch;
pub mod reactive;
pub mod special;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Batch Hack failed.")]
    Batch(#[from] crate::commands::hack::batch::Error),
    #[error("Special Hack failed.")]
    Special(#[from] crate::commands::hack::special::Error),
    #[error("Reactive Hack failed.")]
    Reactive(#[from] crate::commands::hack::reactive::Error),
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Command {
    #[command(visible_alias = "b")]
    Batch(crate::commands::hack::batch::Args),
    #[command(visible_alias = "s")]
    Special(crate::commands::hack::special::Args),
    #[command(visible_alias = "r")]
    Reactive(crate::commands::hack::reactive::Args),
}
impl Command {
    pub(crate) fn run(&self) -> Result<(), Error> {
        use crate::commands::hack::{batch, reactive, special};
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
