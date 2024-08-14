use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;
use console::Term;
use itertools::Itertools;
use regex::Regex;

use crate::{
    config::dirs::tool_workdir,
    process::{run_single, CmdExpression, CmdIoRedirection},
    styled,
};

pub fn beautify_cpp(output: &str, dst: &Path) -> Result<()> {
    log::debug!("Remove duplicate includes.");
    let mut headers: BTreeSet<String> = BTreeSet::new();
    let re = Regex::new(r"#pragma INCLUDE<(.+)>")?;
    for caps in re.captures_iter(output) {
        headers.insert(format!("#include <{}>", caps[1].to_string()));
    }
    log::trace!("headers: {:?}", &headers);
    let output = re.replace_all(&output, "").to_string();
    let output = headers.into_iter().join("\n") + "\n" + &output;
    fs::write(dst, &output)?;

    log::debug!("Apply clang-format.");
    let config_file_path = format_config_path()?;
    let program = "clang-format";
    let args: Vec<String> = vec![
        "-i".into(),
        dst.to_string_lossy().into(),
        format!("--style=file:{}", config_file_path.to_string_lossy()).into(),
    ];
    run_single(
        CmdExpression::new(&program, &args),
        None,
        CmdIoRedirection::default(),
    )?
    .done()?;
    Ok(())
}

fn format_config_path() -> Result<PathBuf> {
    let path = tool_workdir().join(".clang-format");
    if !path.exists() {
        log::warn!(
                "{}",
                styled!(
                    "{} does not exist!\nCreated with default settings. Please edit it!\nPress any key...",
                    path.to_string_lossy()
                )
                .yellow()
            );
        Term::stdout().read_char()?;
        let content = include_str!("../../../data/.clang-format");
        fs::write(&path, &content)?;
    }
    Ok(path)
}
