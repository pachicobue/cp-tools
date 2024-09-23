use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};

use crate::Cli;

/// 補完スクリプトを生成するための引数を格納する構造体
#[derive(Args, Debug)]
pub(crate) struct CompletionArgs {
    /// Shell
    #[arg(required = true, value_enum)]
    pub shell: Shell,
}

/// 補完スクリプトを生成する関数
///
/// # 引数
///
/// * `shell` - シェルの種類
///
/// # 戻り値
///
/// なし
pub(crate) fn print_completion(shell: Shell) {
    let mut app = Cli::command();
    let name = app.get_name().to_string();
    generate(shell, &mut app, name, &mut std::io::stdout());
}
