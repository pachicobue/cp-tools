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

static LANGUAGE_CONFIG: OnceLock<BTreeMap<Language, LanguageConfig>> = OnceLock::new();
static EXT_TO_LANG: OnceLock<BTreeMap<String, Language>> = OnceLock::new();

#[derive(Debug, Clone, strum::EnumString, strum::Display, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Language {
    #[strum(serialize = "cpp")]
    Cpp,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct LanguageConfig {
    name: String,
    extensions: Vec<String>,
    build: Vec<BuildConfig>,
    exec: Vec<ExecConfig>,
    expand: Vec<ExpandConfig>,
}

#[derive(Debug, Clone, Deserialize)]
struct LanguageToml {
    language: Vec<LanguageConfig>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct BuildConfig {
    command: String,
    args: Vec<String>,
    debug_args: Option<Vec<String>>,
    release_args: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ExecConfig {
    command: String,
    args: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ExpandConfig {
    command: String,
    args: Vec<String>,
}

pub(crate) fn init() -> Result<(), LanguageConfigurationError> {
    log::debug!("Loading language.toml ...");
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

pub(crate) fn ensure_buildable(file: &Path) -> Result<(), LanguageSpecificationError> {
    let lang = guess_lang(file)?;
    if get_language_config(lang.clone()).build.is_empty() {
        return Err(LanguageSpecificationError::BuildNotSupported(lang));
    }
    Ok(())
}
pub(crate) fn ensure_executable(file: &Path) -> Result<(), LanguageSpecificationError> {
    let lang = guess_lang(file)?;
    if get_language_config(lang.clone()).exec.is_empty() {
        return Err(LanguageSpecificationError::ExecNotSupported(lang));
    }
    Ok(())
}
pub(crate) fn ensure_expandable(file: &Path) -> Result<(), LanguageSpecificationError> {
    let lang = guess_lang(file)?;
    if get_language_config(lang.clone()).expand.is_empty() {
        return Err(LanguageSpecificationError::ExpandNotSupported(lang));
    }
    Ok(())
}

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

fn get_language_config(lang: Language) -> &'static LanguageConfig {
    LANGUAGE_CONFIG.get().unwrap().get(&lang).unwrap()
}

fn toml_path() -> PathBuf {
    tool_workdir().join("language.toml")
}
