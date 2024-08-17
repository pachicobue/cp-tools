use std::{collections::BTreeSet, fs, path::PathBuf};

use color_eyre::eyre::Result;
use console::Term;
use itertools::Itertools;
use regex::Regex;

use crate::{
    config::dirs::tool_workdir,
    core::{
        fs::{with_tempdir, write_sync},
        process::{run_command_simple, CommandExpression},
    },
    styled,
};

pub fn beautify(output: &str) -> Result<String> {
    log::debug!("Remove system-header contents.");

    // let re = Regex::new(r"#pragma INCLUDE<(.+)>")?;
    // let headers = BTreeSet::from_iter(
    //     re.captures_iter(output)
    //         .map(|caps| format!("#include <{}>", &caps[1])),
    // );
    // log::trace!("headers: {:?}", &headers);
    // let output = headers.iter().join("\n") + "\n" + &re.replace_all(output, "");

    log::debug!("Apply clang-format.");

    let converted_output = with_tempdir(|tempdir| -> Result<String> {
        let temppath = tempdir.path().join("format_src.cpp");
        write_sync(&temppath, &output, true)?;
        let config_file_path = format_config_path()?;
        let program = "clang-format";
        let args: Vec<String> = vec![
            format!("{}", temppath.display()),
            format!("--style=file:{}", config_file_path.to_string_lossy()),
        ];
        Ok(run_command_simple(CommandExpression::new(program, &args))?
            .detail_of_success()?
            .stdout)
    })??;
    Ok(converted_output)
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
        let content = include_str!("../../../resources/cpp/.clang-format");
        fs::write(&path, content)?;
    }
    Ok(path)
}
