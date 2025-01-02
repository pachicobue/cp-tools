pub mod loader;
pub mod parser;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use itertools::Itertools;
use thiserror::Error;

use cpt_stdx::process::CommandExpression;

static LANGUAGE_CONFIG_MAP: OnceLock<ConfigMap> = OnceLock::new();

#[derive(Debug, Clone)]
pub(crate) struct ConfigMap {
    /// lang.name -> config
    pub name_to_config: BTreeMap<String, Config>,
    /// lang.ext  -> lang.name
    pub ext_to_name: BTreeMap<String, String>,
}
#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub build: Option<BuildConfig>,
    pub execute: Option<ExecutionConfig>,
    pub expand: Option<ExpandConfig>,
}
#[derive(Debug, Clone)]
pub(crate) struct BuildConfig {
    pub command: String,
    pub args: Vec<String>,
    pub debug_args: Vec<String>,
    pub release_args: Vec<String>,
}
#[derive(Debug, Clone)]
pub(crate) struct ExecutionConfig {
    pub command: String,
    pub args: Vec<String>,
}
#[derive(Debug, Clone)]
pub(crate) struct ExpandConfig {
    pub command: String,
    pub args: Vec<String>,
}
#[derive(Error, Debug)]
pub(crate) enum LangError {
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
    let language_config_map = parser::merge_parse_tomls(loader::load_tomls());
    LANGUAGE_CONFIG_MAP.set(language_config_map).unwrap();
    log::debug!(
        "LANGUAGE_CONFIG_MAP:\n{:#?}",
        LANGUAGE_CONFIG_MAP.get().unwrap()
    );
}

pub(crate) fn build_command(
    lang: String,
    src_path: &Path,
    bin_path: &Path,
    is_release: bool,
    tmpdir: &Path,
) -> Result<CommandExpression, LangError> {
    let conf = get_language_config(lang.clone())
        .build
        .as_ref()
        .ok_or(LangError::BuildNotSupported(lang))?;
    let mut args = conf.args.clone();
    let extra_args = if is_release {
        conf.clone().release_args
    } else {
        conf.clone().debug_args
    };
    args.extend(extra_args);
    Ok(CommandExpression {
        program: conf.clone().command,
        args: args
            .iter()
            .map(|s| replace(&s, Some(src_path), Some(bin_path), None, tmpdir))
            .collect_vec(),
    })
}

pub(crate) fn exec_command(
    lang: String,
    bin_path: &Path,
    tmpdir: &Path,
) -> Result<CommandExpression, LangError> {
    let conf = get_language_config(lang.to_owned())
        .execute
        .as_ref()
        .ok_or(LangError::ExecNotSupported(lang))?;
    Ok(CommandExpression {
        program: conf.to_owned().command,
        args: conf
            .args
            .iter()
            .map(|s| replace(&s, None, Some(bin_path), None, tmpdir))
            .collect_vec(),
    })
}

pub(crate) fn expand_command(
    lang: String,
    src_path: &Path,
    bundled_path: &Path,
    tmpdir: &Path,
) -> Result<CommandExpression, LangError> {
    let conf = get_language_config(lang.to_owned())
        .expand
        .as_ref()
        .ok_or(LangError::ExpandNotSupported(lang))?;
    Ok(CommandExpression {
        program: conf.to_owned().command,
        args: conf
            .args
            .iter()
            .map(|s| replace(&s, Some(src_path), None, Some(bundled_path), tmpdir))
            .collect_vec(),
    })
}

pub(crate) fn guess_lang(extension: &str) -> Result<String, LangError> {
    let lang = LANGUAGE_CONFIG_MAP
        .get()
        .unwrap()
        .ext_to_name
        .get(extension)
        .ok_or(LangError::ExtensionNotDefined(extension.into()))?;
    Ok(lang.clone())
}

fn replace(
    s: &str,
    src_path: Option<&Path>,
    bin_path: Option<&Path>,
    bundled_path: Option<&Path>,
    tempdir: &Path,
) -> String {
    use crate::dir::workspace_dir;

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
    ns = ns.replace("${tool_workdir}", &workspace_dir().to_string_lossy());
    ns = ns.replace("${tempdir}", &tempdir.to_string_lossy());
    ns
}

fn get_language_config(lang: String) -> &'static Config {
    LANGUAGE_CONFIG_MAP
        .get()
        .unwrap()
        .name_to_config
        .get(&lang)
        .unwrap()
}
