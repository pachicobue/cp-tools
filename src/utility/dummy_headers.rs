use crate::config;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use ::{
    color_eyre::eyre::{ensure, OptionExt, Result},
    duct::cmd,
    regex::Regex,
};

pub(crate) fn generate() -> Result<PathBuf> {
    log::debug!("Generate dummy headers.");
    let dst_dir = config::dirs::workspace_dir().join("dummy_headers");
    let src_dir = Path::new("/usr/include/c++").join(get_gcc_version()?);
    generate_dummy_headers(&dst_dir, &src_dir)?;
    Ok(dst_dir.to_path_buf())
}

fn get_gcc_version() -> Result<String> {
    log::debug!("$ gcc --version");
    let output = cmd!("gcc", "--version").read()?;
    let re = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
    Ok(re
        .captures(&output)
        .ok_or_eyre("Failed to capture gcc version")?[0]
        .to_string())
}

fn generate_dummy_headers(dst_dir: &Path, src_dir: &Path) -> Result<()> {
    log::debug!(
        "Create dummy headers\n{} -> {}",
        src_dir.to_string_lossy(),
        dst_dir.to_string_lossy()
    );
    ensure!(
        src_dir.exists(),
        "Directory {} not found.",
        src_dir.display()
    );

    let mut file_paths = Vec::new();
    log::trace!("original header path: {}", src_dir.display());
    gather_files(src_dir, &mut file_paths)?;
    log::trace!("header_list: {:?}", file_paths);
    for path in file_paths {
        let relative = path.strip_prefix(src_dir).unwrap();
        let dst_path = dst_dir.join(relative);
        fs::create_dir_all(dst_path.parent().unwrap())?;
        let mut f = File::create(&dst_path)?;
        writeln!(f, "#pragma once")?;
        writeln!(f, "#pragma INCLUDE<{}>", relative.display())?;
    }

    Ok(())
}

fn gather_files(src_dir: &Path, file_paths: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(src_dir)? {
        let path = entry?.path();
        if path.is_file() {
            file_paths.push(path);
        } else if path.is_dir() {
            gather_files(&path, file_paths)?;
        }
    }
    Ok(())
}
