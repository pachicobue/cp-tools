pub mod syntax;
pub mod toml_loader;

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use itertools::Itertools;
use syntax::LanguageConfig;
use thiserror::Error;

use crate::config::syntax::LanguageConfigMap;
use crate::core::process::CommandExpression;
use crate::dir;

static LANGUAGE_CONFIG_MAP: OnceLock<LanguageConfigMap> = OnceLock::new();

#[derive(Error, Debug)]
pub(crate) enum ConfigError {
    #[error("Failed to get extension from `{0}`.")]
    ExtensionNotFound(PathBuf),
    #[error("Extention `{0}` is not defined for any language.")]
    ExtensionNotDefined(String),
    #[error("Build command is not supported for language `{0}`.")]
    BuildNotSupported(String),
    #[error("Exec command is not supported for language `{0}`.")]
    ExecNotSupported(String),
    #[error("Expand command is not supported for language `{0}`.")]
    ExpandNotSupported(String),
}

pub(crate) fn init() {
    log::debug!("Initializing language configs...");
    let language_config_map = syntax::language_config_map(toml_loader::load_tomls());
    LANGUAGE_CONFIG_MAP.set(language_config_map).unwrap();
    log::debug!(
        "LANGUAGE_CONFIG_MAP:\n{:#?}",
        LANGUAGE_CONFIG_MAP.get().unwrap()
    );
}

pub(crate) fn ensure_buildable(file: &Path) -> Result<(), ConfigError> {
    let lang = guess_lang(file)?;
    if get_language_config(lang.clone()).build.is_none() {
        return Err(ConfigError::BuildNotSupported(lang));
    }
    Ok(())
}

pub(crate) fn ensure_executable(file: &Path) -> Result<(), ConfigError> {
    let lang = guess_lang(file)?;
    if get_language_config(lang.clone()).execute.is_none() {
        return Err(ConfigError::ExecNotSupported(lang));
    }
    Ok(())
}

pub(crate) fn ensure_expandable(file: &Path) -> Result<(), ConfigError> {
    let lang = guess_lang(file)?;
    if get_language_config(lang.clone()).expand.is_none() {
        return Err(ConfigError::ExpandNotSupported(lang));
    }
    Ok(())
}

pub(crate) fn build_command(
    lang: String,
    src_path: &Path,
    bin_path: &Path,
    is_release: bool,
    tmpdir: &Path,
) -> CommandExpression {
    let conf = get_language_config(lang.clone()).build.as_ref().unwrap();
    let mut args = conf.args.clone();
    let extra_args = if is_release {
        conf.clone().release_args
    } else {
        conf.clone().debug_args
    };
    args.extend(extra_args);
    CommandExpression {
        program: conf.clone().command,
        args: args
            .iter()
            .map(|s| replace(&s, Some(src_path), Some(bin_path), None, tmpdir))
            .collect_vec(),
    }
}

pub(crate) fn exec_command(lang: String, bin_path: &Path, tmpdir: &Path) -> CommandExpression {
    let conf = get_language_config(lang.clone()).execute.as_ref().unwrap();
    CommandExpression {
        program: conf.clone().command,
        args: conf
            .args
            .iter()
            .map(|s| replace(&s, None, Some(bin_path), None, tmpdir))
            .collect_vec(),
    }
}

pub(crate) fn expand_command(
    lang: String,
    src_path: &Path,
    bundled_path: &Path,
    tmpdir: &Path,
) -> CommandExpression {
    let conf = get_language_config(lang.clone()).expand.as_ref().unwrap();
    CommandExpression {
        program: conf.clone().command,
        args: conf
            .args
            .iter()
            .map(|s| replace(&s, Some(src_path), None, Some(bundled_path), tmpdir))
            .collect_vec(),
    }
}

pub(crate) fn guess_lang(src_path: &Path) -> Result<String, ConfigError> {
    let ext = src_path
        .extension()
        .ok_or_else(|| ConfigError::ExtensionNotFound(src_path.to_path_buf()))?
        .to_string_lossy()
        .to_string();
    let lang = LANGUAGE_CONFIG_MAP
        .get()
        .unwrap()
        .ext_to_name
        .get(&ext)
        .ok_or_else(|| ConfigError::ExtensionNotDefined(ext))?;
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
    ns = ns.replace("${tool_workdir}", &dir::workspace_dir().to_string_lossy());
    ns = ns.replace("${tempdir}", &tempdir.to_string_lossy());
    ns
}

fn get_language_config(lang: String) -> &'static LanguageConfig {
    LANGUAGE_CONFIG_MAP
        .get()
        .unwrap()
        .name_to_config
        .get(&lang)
        .unwrap()
}
