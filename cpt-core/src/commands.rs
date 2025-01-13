mod batch_test;

#[derive(thiserror::Error, Debug)]
pub(super) enum Error {
    #[error("BatchTest command failed.")]
    BatchTestFailed(#[from] crate::commands::batch_test::Error),
}

#[derive(clap::Subcommand, Debug)]
pub(super) enum Command {
    #[command(visible_alias = "tb")]
    BatchTest(crate::commands::batch_test::Args),
}

impl Command {
    pub(super) fn run(&self) -> Result<(), Error> {
        use crate::commands::batch_test;
        match self {
            Command::BatchTest(args) => {
                batch_test::run(args)?;
            }
        }
        Ok(())
    }
}
