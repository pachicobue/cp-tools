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
    anyhow::Result,
    clap::{Parser, Subcommand},
};

/// コマンドライン引数
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
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
    ///　オンラインジャッジにログインする
    Login(LoginArgs),
    /// シェル補完関数を生成する
    Completion(CompletionArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    config::init()?;

    let args = Cli::parse();
    match args.command {
        Commands::Expand(args) => {
            expand(&args).await?;
        }
        Commands::Build(args) => {
            build(&args).await?;
        }
        Commands::Download(args) => {
            download(&args).await?;
        }
        Commands::Login(args) => {
            login(&args).await?;
        }
        Commands::Completion(args) => {
            print_completion(args.shell);
        }
    }
    Ok(())
}
