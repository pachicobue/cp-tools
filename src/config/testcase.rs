use std::path::{Path, PathBuf};

use color_eyre::eyre::{OptionExt, Result};
use walkdir::WalkDir;

use crate::{config::dirs::project_workdir, judge::Testcase};

const INPUT_EXT: &str = "in";
const OUTPUT_EXT: &str = "out";

pub(crate) fn collect_testcases(dir: &Path) -> Vec<Testcase> {
    let mut cases: Vec<Testcase> = Vec::new();
    for entry in WalkDir::new(dir).max_depth(1).into_iter().filter(|entry| {
        entry
            .as_ref()
            .is_ok_and(|entry| entry.path().extension().unwrap_or_default() == INPUT_EXT)
    }) {
        match entry {
            Ok(entry) => {
                let input = entry.path().to_path_buf();
                let output = input.with_extension(OUTPUT_EXT);
                cases.push(Testcase {
                    input,
                    expect: if output.exists() { Some(output) } else { None },
                    actual: None,
                });
            }
            Err(e) => log::error!("{}", e),
        }
    }
    cases
}

pub(crate) fn default_casedir(file: &Path) -> Result<PathBuf> {
    let basedir = project_workdir(file.parent().ok_or_eyre("Failed to get parent")?)?;
    let name = file
        .file_stem()
        .ok_or_eyre("Failed to get stem.")?
        .to_string_lossy()
        .to_string();
    Ok(basedir.join(name + "_cases"))
}
