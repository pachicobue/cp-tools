use std::collections::BTreeMap;

use serde::Deserialize;
use thiserror::Error;

use crate::lang::{BuildConfig, Config, ConfigMap, ExecutionConfig, ExpandConfig};

#[derive(Debug, Error)]
enum SyntaxError {
    #[error("Failed to parse toml into language configuration.")]
    ParseError(#[source] toml::de::Error),
}

#[derive(Debug, Clone, Default, Deserialize)]
struct LanguageToml {
    language: Vec<ConfigToml>,
}
#[derive(Debug, Clone, Default, Deserialize)]
struct ConfigToml {
    name: String,
    extensions: Vec<String>,
    build: Option<BuildConfigToml>,
    execute: Option<ExecuteConfigToml>,
    expand: Option<ExpandConfigToml>,
}
#[derive(Debug, Clone, Default, Deserialize)]
struct BuildConfigToml {
    command: String,
    args: Vec<String>,
    debug_args: Option<Vec<String>>,
    release_args: Option<Vec<String>>,
}
#[derive(Debug, Clone, Default, Deserialize)]
struct ExecuteConfigToml {
    command: String,
    args: Vec<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
struct ExpandConfigToml {
    command: String,
    args: Vec<String>,
}

pub(crate) fn merge_parse_tomls(tomls: Vec<toml::Value>) -> ConfigMap {
    let raw = tomls
        .iter()
        .filter_map(|toml| parse_toml(toml.to_owned()).ok())
        .fold(LanguageToml::default(), merge);
    convert(raw)
}

fn convert_single(raw_config: ConfigToml) -> Config {
    let ConfigToml {
        name: _,
        extensions: _,
        build,
        execute,
        expand,
    } = raw_config;
    Config {
        build: build.map(
            |BuildConfigToml {
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
            .map(|ExecuteConfigToml { command, args }| ExecutionConfig { command, args }),
        expand: expand.map(|ExpandConfigToml { command, args }| ExpandConfig { command, args }),
    }
}

fn convert(raw_configs: LanguageToml) -> ConfigMap {
    let mut name_to_config = BTreeMap::<String, Config>::new();
    let mut ext_to_name = BTreeMap::<String, String>::new();
    raw_configs.language.into_iter().for_each(|raw_config| {
        let name = raw_config.to_owned().name;
        let config = convert_single(raw_config.to_owned());
        name_to_config.insert(name.to_owned(), config);
        for ext in raw_config.extensions {
            ext_to_name.insert(ext, name.to_owned());
        }
    });
    ConfigMap {
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
