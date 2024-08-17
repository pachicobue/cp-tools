use std::path::Path;

use color_eyre::eyre::{ensure, Context, ContextCompat, OptionExt, Result};
use tempfile::{self, TempDir};
use tokio;

use crate::config::metadata::CRATE_NAME;

pub(crate) fn read_sync(filepath: impl AsRef<Path>) -> Result<String> {
    ensure!(
        filepath.as_ref().exists(),
        "`{}` does not exist.",
        filepath.as_ref().display()
    );
    let content = std::fs::read_to_string(filepath.as_ref())
        .wrap_err(format!("Failed to read `{}`.", filepath.as_ref().display()))?;
    Ok(content)
}

pub(crate) fn write_sync(
    filepath: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
    ensure_exist: bool,
) -> Result<()> {
    if ensure_exist {
        let dir = filepath.as_ref().parent().ok_or_eyre(format!(
            "Failed to get parent of `{}`.",
            filepath.as_ref().display()
        ))?;
        std::fs::create_dir_all(dir).wrap_err(format!(
            "Failed to create directory `{}` recursively.",
            dir.display()
        ))?;
    }
    std::fs::write(filepath.as_ref(), &content).wrap_err(format!(
        "Failed to write `{}`.",
        filepath.as_ref().display()
    ))?;
    Ok(())
}

pub(crate) fn open_sync(filepath: impl AsRef<Path>) -> Result<std::fs::File> {
    std::fs::File::open(filepath.as_ref()).wrap_err(format!(
        "Failed to open file `{}`.",
        filepath.as_ref().display()
    ))
}

pub(crate) fn create_sync(filepath: impl AsRef<Path>) -> Result<std::fs::File> {
    std::fs::File::create(filepath.as_ref()).wrap_err(format!(
        "Failed to create file `{}`.",
        filepath.as_ref().display()
    ))
}

pub(crate) async fn read_async(filepath: impl AsRef<Path>) -> Result<String> {
    ensure!(
        filepath.as_ref().exists(),
        "`{}` does not exist.",
        filepath.as_ref().display()
    );
    let content = tokio::fs::read_to_string(filepath.as_ref())
        .await
        .wrap_err(format!("Failed to read `{}`.", filepath.as_ref().display()))?;
    Ok(content)
}

pub(crate) async fn write_async(
    filepath: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
    ensure_exist: bool,
) -> Result<()> {
    if ensure_exist {
        let dir = filepath.as_ref().parent().ok_or_eyre(format!(
            "Failed to get parent of `{}`.",
            filepath.as_ref().display()
        ))?;
        tokio::fs::create_dir_all(dir)
            .await
            .wrap_err(format!("Failed to create `{}` recursively.", dir.display()))?;
    }
    tokio::fs::write(filepath.as_ref(), &content)
        .await
        .wrap_err(format!(
            "Failed to write `{}`.",
            filepath.as_ref().display()
        ))?;
    Ok(())
}

pub(crate) async fn create_async(filepath: impl AsRef<Path>) -> Result<tokio::fs::File> {
    tokio::fs::File::create(filepath.as_ref())
        .await
        .wrap_err(format!(
            "Failed to create file `{}`.",
            filepath.as_ref().display()
        ))
}

pub(crate) async fn open_async(filepath: impl AsRef<Path>) -> Result<tokio::fs::File> {
    tokio::fs::File::open(filepath.as_ref())
        .await
        .wrap_err(format!(
            "Failed to open file `{}`.",
            filepath.as_ref().display()
        ))
}

pub(crate) fn filename(filepath: impl AsRef<Path>) -> Result<String> {
    Ok(filepath
        .as_ref()
        .file_stem()
        .wrap_err(format!(
            "Failed to get filestem from `{}`",
            filepath.as_ref().display(),
        ))?
        .to_string_lossy()
        .to_string())
}

pub(crate) fn with_tempdir<F, R>(func: F) -> Result<R>
where
    F: FnOnce(&TempDir) -> R,
{
    let tempdir = tempfile::Builder::new()
        .prefix(&format!("{}-", CRATE_NAME))
        .tempdir()
        .wrap_err("Failed to build tempdir.")?;
    let result = func(&tempdir);
    tempdir.close().wrap_err("Failed to close tempdir.")?;
    Ok(result)
}
