use log::LevelFilter;

use crate::config::metadata::CRATE_NAME;

/// ロガーを初期化する関数
///
/// # 引数
///
/// * `level` - ログのレベル
///
/// # 戻り値
///
/// なし
pub fn init(level: LevelFilter) {
    env_logger::init();
}
