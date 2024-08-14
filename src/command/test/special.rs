use std::path::PathBuf;

use clap::{Args, ValueHint};
use color_eyre::eyre::Result;

#[derive(Args, Debug)]
pub(crate) struct SpecialArgs {
    /// 入力ファイル(.cppのみ対応)
    #[clap(required = true, value_hint(ValueHint::FilePath))]
    file: PathBuf,

    /// テストディレクトリ
    #[arg(short = 'd', long, value_hint(ValueHint::FilePath))]
    directory: Option<PathBuf>,

    /// コンパイル最適化フラグ
    #[arg(long, default_value_t = false)]
    release: bool,
}
pub(crate) fn special(args: &SpecialArgs) -> Result<()> {
    Ok(())
}
