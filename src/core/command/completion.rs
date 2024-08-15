use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};

use crate::Cli;

#[derive(Args, Debug)]
pub(crate) struct CompletionArgs {
    /// Shell
    #[arg(required = true, value_enum)]
    pub shell: Shell,
}

pub(crate) fn print_completion(shell: Shell) {
    let mut app = Cli::command();
    let name = app.get_name().to_string();
    generate(shell, &mut app, name, &mut std::io::stdout());
}
