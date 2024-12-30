use std::{
    collections::BTreeMap,
    ffi::OsString,
    path::{Path, PathBuf},
    str::FromStr,
    sync::OnceLock,
};

use itertools::Itertools;
use serde::Deserialize;
use strum;

use super::{error::LanguageSpecificationError, process::CommandExpression};
use crate::{
    config::dirs::tool_workdir,
    core::{error::LanguageConfigurationError, fs::read_sync},
};

/// 言語設定を格納する静的変数
static LANGUAGE_CONFIG: OnceLock<BTreeMap<Language, LanguageConfig>> = OnceLock::new();
/// 拡張子から言語へのマッピングを格納する静的変数
static EXT_TO_LANG: OnceLock<BTreeMap<String, Language>> = OnceLock::new();

/// プログラミング言語を表す列挙型
#[derive(Debug, Clone, strum::EnumString, strum::Display, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Language {
    /// C++
    #[strum(serialize = "cpp")]
    Cpp,
}

/// 言語の設定を格納する構造体
#[derive(Debug, Clone, Default, Deserialize)]
struct LanguageConfig {
    /// 言語の名前
    name: String,
    /// 対応するファイル拡張子のリスト
    extensions: Vec<String>,
    /// ビルド設定のリスト
    build: Vec<BuildConfig>,
    /// 実行設定のリスト
    exec: Vec<ExecConfig>,
    /// 展開設定のリスト
    expand: Vec<ExpandConfig>,
}

/// TOMLファイルから読み込まれる言語設定を格納する構造体
#[derive(Debug, Clone, Deserialize)]
struct LanguageToml {
    /// 言語設定のリスト
    language: Vec<LanguageConfig>,
}

/// ビルド設定を格納する構造体
#[derive(Debug, Clone, Default, Deserialize)]
struct BuildConfig {
    /// コマンド
    command: String,
    /// 引数のリスト
    args: Vec<String>,
    /// デバッグ時の追加引数のリスト
    debug_args: Option<Vec<String>>,
    /// リリース時の追加引数のリスト
    release_args: Option<Vec<String>>,
}

/// 実行設定を格納する構造体
#[derive(Debug, Clone, Default, Deserialize)]
struct ExecConfig {
    /// コマンド
    command: String,
    /// 引数のリスト
    args: Vec<String>,
}

/// 展開設定を格納する構造体
#[derive(Debug, Clone, Default, Deserialize)]
struct ExpandConfig {
    /// コマンド
    command: String,
    /// 引数のリスト
    args: Vec<String>,
}

/// 言語設定を初期化する関数
///
/// # 戻り値
///
/// 言語設定の初期化に成功した場合は`Ok(())`、失敗した場合は`LanguageConfigurationError`を返す
pub(crate) fn init() -> Result<(), LanguageConfigurationError> {
    log::debug!("Loading language.toml ...");
    log::debug!("{}", read_sync(toml_path()).unwrap().to_string());
    let toml: LanguageToml = toml::from_str(&read_sync(toml_path()).unwrap())
        .map_err(|e| LanguageConfigurationError::TomlParseError(e, toml_path()))?;
    log::trace!("{:#?}", toml);
    let mut tmp_lang_config = BTreeMap::<Language, LanguageConfig>::new();
    let mut tmp_ext_to_lang = BTreeMap::<String, Language>::new();
    for language_config in toml.language {
        let lang = Language::from_str(&language_config.name).map_err(|e| {
            LanguageConfigurationError::LanguageNotSupportedError(e, language_config.name.clone())
        })?;
        tmp_lang_config.insert(lang.clone(), language_config.clone());
        for ext in language_config.extensions {
            if tmp_ext_to_lang
                .get(&ext)
                .is_some_and(|prev_lang| prev_lang.to_owned() != lang)
            {
                return Err(LanguageConfigurationError::DuplicateExtensionError(ext));
            }
            tmp_ext_to_lang.insert(ext, lang.clone());
        }
    }
    LANGUAGE_CONFIG.get_or_init(|| tmp_lang_config);
    EXT_TO_LANG.get_or_init(|| tmp_ext_to_lang);
    log::trace!("Language config: {:#?}", LANGUAGE_CONFIG.get().unwrap());
    log::trace!("Extension to lang: {:#?}", EXT_TO_LANG.get().unwrap());
    Ok(())
}

/// ファイルがビルド可能かどうかを確認する関数
///
/// # 引数
///
/// * `file` - 確認するファイルのパス
///
/// # 戻り値
///
/// ビルド��能な場合は`Ok(())`、不可能な場合は`LanguageSpecificationError`を返す
pub(crate) fn ensure_buildable(file: &Path) -> Result<(), LanguageSpecificationError> {
    let lang = guess_lang(file)?;
    if get_language_config(lang.clone()).build.is_empty() {
        return Err(LanguageSpecificationError::BuildNotSupported(lang));
    }
    Ok(())
}

/// ファイルが実行可能かどうかを確認する関数
///
/// # 引数
///
/// * `file` - 確認するファイルのパス
///
/// # 戻り値
///
/// 実行可能な場合は`Ok(())`、不可能な場合は`LanguageSpecificationError`を返す
pub(crate) fn ensure_executable(file: &Path) -> Result<(), LanguageSpecificationError> {
    let lang = guess_lang(file)?;
    if get_language_config(lang.clone()).exec.is_empty() {
        return Err(LanguageSpecificationError::ExecNotSupported(lang));
    }
    Ok(())
}

/// ファイルが展開可能かどうかを確認する関数
///
/// # 引数
///
/// * `file` - 確認するファイルのパス
///
/// # 戻り値
///
/// 展開可能な場合は`Ok(())`、不可能な場合は`LanguageSpecificationError`を返す
pub(crate) fn ensure_expandable(file: &Path) -> Result<(), LanguageSpecificationError> {
    let lang = guess_lang(file)?;
    if get_language_config(lang.clone()).expand.is_empty() {
        return Err(LanguageSpecificationError::ExpandNotSupported(lang));
    }
    Ok(())
}

/// ビルドコマンドを生成する関数
///
/// # 引数
///
/// * `lang` - プログラミング言語
/// * `src_path` - ソースファイルのパス
/// * `bin_path` - バイナリファイルのパス
/// * `is_release` - リリースビルドかどうか
/// * `tmpdir` - 一時ディレクトリのパス
///
/// # 戻り値
///
/// `CommandExpression`のベクター
pub(crate) fn build_command(
    lang: Language,
    src_path: &Path,
    bin_path: &Path,
    is_release: bool,
    tmpdir: &Path,
) -> Vec<CommandExpression> {
    get_language_config(lang.clone())
        .build
        .iter()
        .map(|conf| {
            let mut args = conf.args.clone();
            let extra_args = if is_release {
                conf.clone().release_args.unwrap_or_default()
            } else {
                conf.clone().debug_args.unwrap_or_default()
            };
            args.extend(extra_args);
            CommandExpression {
                program: conf.clone().command.into(),
                args: args
                    .iter()
                    .map(|s| replace(&s, Some(src_path), Some(bin_path), None, tmpdir))
                    .map_into::<OsString>()
                    .collect_vec(),
            }
        })
        .collect_vec()
}

/// 実行コマンドを生成する関数
///
/// # 引数
///
/// * `lang` - プログラミング言語
/// * `bin_path` - バイナリファイルのパス
/// * `tmpdir` - 一時ディレクトリのパス
///
/// # 戻り値
///
/// `CommandExpression`のベクター
pub(crate) fn exec_command(
    lang: Language,
    bin_path: &Path,
    tmpdir: &Path,
) -> Vec<CommandExpression> {
    get_language_config(lang.clone())
        .exec
        .iter()
        .map(|conf| CommandExpression {
            program: conf.clone().command.into(),
            args: conf
                .args
                .iter()
                .map(|s| replace(&s, None, Some(bin_path), None, tmpdir))
                .map_into::<OsString>()
                .collect_vec(),
        })
        .collect_vec()
}

/// 展開コマンドを生成する関数
///
/// # 引数
///
/// * `lang` - プログラミング言語
/// * `src_path` - ソースファイルのパス
/// * `bundled_path` - バンドルファイルのパス
/// * `tmpdir` - 一時ディレクトリのパス
///
/// # 戻り値
///
/// `CommandExpression`のベクター
pub(crate) fn expand_command(
    lang: Language,
    src_path: &Path,
    bundled_path: &Path,
    tmpdir: &Path,
) -> Vec<CommandExpression> {
    get_language_config(lang.clone())
        .expand
        .iter()
        .map(|conf| CommandExpression {
            program: conf.clone().command.into(),
            args: conf
                .args
                .iter()
                .map(|s| replace(&s, Some(src_path), None, Some(bundled_path), tmpdir))
                .map_into::<OsString>()
                .collect_vec(),
        })
        .collect_vec()
}

/// ファイルの拡張子からプログラミング言語を推測する関数
///
/// # 引数
///
/// * `src_path` - ソースファイルのパス
///
/// # 戻り値
///
/// 推測されたプログラミング言語、または`LanguageSpecificationError`
pub(crate) fn guess_lang(src_path: &Path) -> Result<Language, LanguageSpecificationError> {
    let ext = src_path
        .extension()
        .ok_or_else(|| LanguageSpecificationError::ExtensionIsNotFound(src_path.to_path_buf()))?
        .to_string_lossy()
        .to_string();
    let lang = EXT_TO_LANG
        .get()
        .unwrap()
        .get(&ext)
        .ok_or_else(|| LanguageSpecificationError::LanguageNotDefined(ext))?;
    Ok(lang.clone())
}

/// 文字列内のプレースホルダーを置換する関数
///
/// # 引数
///
/// * `s` - 置換対象の文字列
/// * `src_path` - ソースファイルのパス
/// * `bin_path` - バイナリファイルのパス
/// * `bundled_path` - バンドルファイルのパス
/// * `tempdir` - 一時ディレクトリのパス
///
/// # 戻り値
///
/// 置換後の文字列
fn replace(
    s: &str,
    src_path: Option<&Path>,
    bin_path: Option<&Path>,
    bundled_path: Option<&Path>,
    tempdir: &Path,
) -> String {
    let mut ns = s.to_string();
    if let Some(src) = src_path {
        ns = ns.replace("${src_path}", &src.to_string_lossy());
    }
    if let Some(bin) = bin_path {
        ns = ns.replace("${bin_path}", &bin.to_string_lossy());
    }
    if let Some(bundled) = bundled_path {
        ns = ns.replace("${bundled_path}", &bundled.to_string_lossy());
    }
    ns = ns.replace("${tool_workdir}", &tool_workdir().to_string_lossy());
    ns = ns.replace("${tempdir}", &tempdir.to_string_lossy());
    ns
}

/// 指定された言語の設定を取得する関数
///
/// # 引数
///
/// * `lang` - プログラミング言語
///
/// # 戻り値
///
/// 言語設定の参照
fn get_language_config(lang: Language) -> &'static LanguageConfig {
    LANGUAGE_CONFIG.get().unwrap().get(&lang).unwrap()
}

/// TOMLファイルのパスを取得する関数
///
/// # 戻り値
///
/// TOMLファイルのパス
fn toml_path() -> PathBuf {
    tool_workdir().join("language.toml")
}
