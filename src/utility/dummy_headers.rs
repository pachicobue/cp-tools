use crate::config;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use ::{
    anyhow::{ensure, Result},
    regex::Regex,
    tokio::process::Command,
};

pub(crate) async fn generate() -> Result<PathBuf> {
    let dst_dir = config::dirs::workspace_dir().join("dummy_headers");
    let src_dir = Path::new("/usr/include/c++").join(get_gcc_version().await?);
    generate_dummy_headers(&dst_dir, &src_dir)?;
    Ok(dst_dir.to_path_buf())
}

async fn get_gcc_version() -> Result<String> {
    let output = Command::new("gcc").arg("--version").output().await?.stdout;
    let output = String::from_utf8(output).unwrap();
    let re: Regex = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
    Ok(re.captures(&output).unwrap()[0].to_string())
}

fn generate_dummy_headers(dst_dir: &Path, src_dir: &Path) -> Result<()> {
    ensure!(
        src_dir.exists(),
        "Directory {} not found.",
        src_dir.display()
    );

    let mut file_paths = Vec::new();
    log::info!("Gathering headers from {} ...", src_dir.display());
    gather_files(src_dir, &mut file_paths)?;
    log::debug!("Gatherd headers: {:?}", file_paths);
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
