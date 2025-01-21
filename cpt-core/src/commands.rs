mod hack;
mod test;

#[derive(thiserror::Error, Debug)]
pub(super) enum Error {
    #[error("Test failed.")]
    TestFailed(#[from] crate::commands::test::Error),
    #[error("Hack failed.")]
    HackFailed(#[from] crate::commands::hack::Error),
}

#[derive(clap::Subcommand, Debug)]
pub(super) enum Command {
    #[command(subcommand, visible_alias = "t")]
    Test(crate::commands::test::Command),
    #[command(subcommand, visible_alias = "h")]
    Hack(crate::commands::hack::Command),
}

impl Command {
    pub(super) fn run(&self) -> Result<(), Error> {
        match self {
            Command::Test(command) => {
                command.run()?;
            }
            Command::Hack(command) => {
                command.run()?;
            }
        }
        Ok(())
    }
}
