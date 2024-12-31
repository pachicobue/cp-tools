use std::collections::BTreeMap;

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone)]
pub(crate) struct LanguageConfigMap {
    /// lang.name -> config
    pub name_to_config: BTreeMap<String, LanguageConfig>,
    /// lang.ext  -> lang.name
    pub ext_to_name: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub(crate) struct LanguageConfig {
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
#[derive(Debug, Error)]
enum SyntaxError {
    #[error("Failed to parse toml into language configuration.")]
    ParseError(#[source] toml::de::Error),
}

#[derive(Debug, Clone, Default, Deserialize)]
struct LanguageToml {
    language: Vec<ConfigRaw>,
}
#[derive(Debug, Clone, Default, Deserialize)]
struct ConfigRaw {
    name: String,
    extensions: Vec<String>,
    build: Option<BuildConfigRaw>,
    execute: Option<ExecuteConfigRaw>,
    expand: Option<ExpandConfigRaw>,
}
#[derive(Debug, Clone, Default, Deserialize)]
struct BuildConfigRaw {
    command: String,
    args: Vec<String>,
    debug_args: Option<Vec<String>>,
    release_args: Option<Vec<String>>,
}
#[derive(Debug, Clone, Default, Deserialize)]
struct ExecuteConfigRaw {
    command: String,
    args: Vec<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
struct ExpandConfigRaw {
    command: String,
    args: Vec<String>,
}

pub(crate) fn language_config_map(tomls: Vec<toml::Value>) -> LanguageConfigMap {
    let raw = tomls
        .iter()
        .filter_map(|toml| parse_toml(toml.to_owned()).ok())
        .fold(LanguageToml::default(), merge);
    convert(raw)
}

fn convert_single(raw_config: ConfigRaw) -> LanguageConfig {
    let ConfigRaw {
        name: _,
        extensions: _,
        build,
        execute,
        expand,
    } = raw_config;
    LanguageConfig {
        build: build.map(
            |BuildConfigRaw {
                 command,
                 args,
                 debug_args,
                 release_args,
             }| BuildConfig {
                command,
                args,
                debug_args: debug_args.to_owned().unwrap_or(vec![]),
                release_args: release_args.to_owned().unwrap_or(vec![]),
            },
        ),
        execute: execute
            .map(|ExecuteConfigRaw { command, args }| ExecutionConfig { command, args }),
        expand: expand.map(|ExpandConfigRaw { command, args }| ExpandConfig { command, args }),
    }
}

fn convert(raw_configs: LanguageToml) -> LanguageConfigMap {
    let mut name_to_config = BTreeMap::<String, LanguageConfig>::new();
    let mut ext_to_name = BTreeMap::<String, String>::new();
    raw_configs.language.into_iter().for_each(|raw_config| {
        let name = raw_config.to_owned().name;
        let config = convert_single(raw_config.to_owned());
        name_to_config.insert(name.to_owned(), config);
        for ext in raw_config.extensions {
            ext_to_name.insert(ext, name.to_owned());
        }
    });
    LanguageConfigMap {
        name_to_config,
        ext_to_name,
    }
}

/// Parse toml into language configurations.
fn parse_toml(toml: toml::Value) -> Result<LanguageToml, SyntaxError> {
    log::debug!("Parsing toml string...\n {}", toml);
    let res = toml.try_into().map_err(|e| SyntaxError::ParseError(e));
    log::debug!("result:\n {:#?}", res);
    res
}

/// Merge two languages.toml configurations.
/// If same language.name exists in both config, older config's whole [[language]] section gets lost.
///
/// old:
///   [[language]]
///   name = "cpp"
///   extensions = ["cpp", "hpp"]
///   [build]
///   command = "clang++"
///   args = []
///   [[language]]
///   name = "nix"
///   extensions = ["nix"]
///
/// new:
///   [[language]]
///   name = "cpp"
///   extensions = ["cpp"]
///   [language.expand]
///   command = "echo"
///   args = []
///
/// merged:
///   [[language]]
///   name = "cpp"
///   extensions = ["cpp"]
///   [language.expand]
///   command = "echo"
///   args = []
///   [[language]]
///   name = "nix"
///   extensions = ["nix"]
fn merge(old: LanguageToml, new: LanguageToml) -> LanguageToml {
    let mut merged = new.clone();
    for conf in old.language.iter() {
        if !merged.language.iter().any(|c| c.name == conf.name) {
            merged.language.push(conf.to_owned());
        }
    }
    merged
}
