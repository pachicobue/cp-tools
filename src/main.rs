mod config;
mod utility;

mod command;

mod judge;
mod oj_client;

use crate::{
    command::build::{build, BuildArgs},
    command::completion::{print_completion, CompletionArgs},
    command::download::{download, DownloadArgs},
    command::expand::{expand, ExpandArgs},
    command::login::{login, LoginArgs},
};
use ::{
    clap::{Parser, Subcommand},
    clap_verbosity_flag::{InfoLevel, Verbosity},
    color_eyre::eyre::Result,
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
    /// ソースコード中の#includeを展開する
    Expand(ExpandArgs),
    /// ソースコードをビルドする
    Build(BuildArgs),
    /// サンプルケースをダウンロードする
    Download(DownloadArgs),
    /// オンラインジャッジにログインする
    Login(LoginArgs),
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
        Commands::Download(args) => {
            download(&args)?;
        }
        Commands::Login(args) => {
            login(&args)?;
        }
        Commands::Completion(args) => {
            print_completion(args.shell);
        }
    }
    Ok(())
}
