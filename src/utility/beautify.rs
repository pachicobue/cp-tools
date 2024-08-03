use itertools::Itertools;

use crate::config;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use ::{color_eyre::eyre::Result, regex::Regex};

pub fn beautify(output: &str, dst: &Path) -> Result<()> {
    log::debug!("Remove duplicate includes.");
    let mut headers: HashSet<String> = HashSet::new();
    let re = Regex::new(r"#pragma INCLUDE<(.+)>")?;
    for caps in re.captures_iter(output) {
        headers.insert(format!("#include <{}>", caps[1].to_string()));
    }
    log::trace!("headers: {:?}", &headers);
    let output = re.replace_all(&output, "").to_string();
    let output = headers.into_iter().join("\n") + "\n" + &output;
    fs::write(dst, &output)?;

    log::debug!("Apply clang-format.");
    let config_file_path = default_clang_format_config();
    let program = "clang-format";
    let mut args: Vec<String> = vec!["-i".to_string(), dst.to_string_lossy().into()];
    if config_file_path.exists() {
        args.push(format!("--style=file:{}", config_file_path.to_string_lossy()).into());
    } else {
        log::warn!("Could not find default .clang-format! Use system default.");
    }
    log::debug!("$ {} {}", &program, &args.join(" "));
    let cmd_output = duct::cmd(program, &args).read()?;
    log::trace!("{}", cmd_output);
    Ok(())
}

fn default_clang_format_config() -> PathBuf {
    config::dirs::workspace_dir().join(".clang-format")
}
