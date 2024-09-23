pub mod dirs;
pub mod logger;
pub mod metadata;

use log::LevelFilter;

use crate::core::{error::InitializationError, language};

/// 設定を初期化する関数
///
/// # 引数
///
/// * `level` - ログのレベル
///
/// # 戻り値
///
/// 設定の初期化結果
pub(crate) fn init(level: LevelFilter) -> Result<(), InitializationError> {
    logger::init(level);
    dirs::init();
    language::init()?;
    Ok(())
}
