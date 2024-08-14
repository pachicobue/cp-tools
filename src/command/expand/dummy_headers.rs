use std::path::{Path, PathBuf};

use color_eyre::eyre::{ensure, Context, OptionExt, Result};
use itertools::Itertools;
use regex::Regex;
use walkdir::WalkDir;

use crate::{
    config::dirs::tool_workdir,
    fs::write,
    process::{run_single, CmdExpression, CmdIoRedirection},
};

pub(crate) fn generate() -> Result<PathBuf> {
    log::debug!("Generate dummy headers.");
    let version = get_gcc_version()?;
    let src_dir = input_dir(&version);
    let dst_dir = output_dir(&version);
    generate_dummy_headers(&dst_dir, &src_dir)?;
    Ok(dst_dir.to_path_buf())
}

fn get_gcc_version() -> Result<String> {
    let output = run_single(
        CmdExpression::new("gcc", ["--version"]),
        None,
        CmdIoRedirection::default(),
    )?
    .done()?
    .stdout;
    let re = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
    Ok(re
        .captures(&output)
        .ok_or_eyre(format!("Failed to capture gcc version from `{}`.", &output))?[0]
        .into())
}

fn generate_dummy_headers(dst_dir: &Path, src_dir: &Path) -> Result<()> {
    log::debug!(
        "Create dummy headers\nSrc dir: `{}`\nDst dir: `{}`",
        src_dir.display(),
        dst_dir.display(),
    );
    ensure!(
        src_dir.exists(),
        "Directory `{}` not found.",
        src_dir.display()
    );

    let file_paths = gather_files(src_dir);
    for path in file_paths {
        let relative = path.strip_prefix(src_dir).context(format!(
            "Failed to strip `{}` from `{}`.",
            src_dir.display(),
            path.display()
        ))?;
        let dst_path = dst_dir.join(relative);
        if !dst_path.exists() {
            log::trace!("create `{}`", dst_path.display());
            write(
                &dst_path,
                format!("#pragma once\n#pragma INCLUDE<{}>\n", relative.display()),
                true,
            )?;
        }
    }

    Ok(())
}

fn gather_files(src_dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(src_dir)
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().into_path().to_path_buf())
        .collect_vec()
}

fn input_dir(version: &str) -> PathBuf {
    Path::new("/usr/include/c++").join(version)
}

fn output_dir(version: &str) -> PathBuf {
    tool_workdir().join("dummy_headers").join(version)
}
