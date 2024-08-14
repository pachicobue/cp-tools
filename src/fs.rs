use std::path::Path;

use color_eyre::eyre::{ensure, Context, OptionExt, Result};
use tokio;

pub(crate) fn read(filepath: impl AsRef<Path>) -> Result<String> {
    ensure!(
        filepath.as_ref().exists(),
        "`{}` does not exist.",
        filepath.as_ref().display()
    );
    let content = std::fs::read_to_string(filepath.as_ref())
        .context(format!("Failed to read `{}`.", filepath.as_ref().display()))?;
    Ok(content)
}

pub(crate) fn write(
    filepath: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
    ensure_exist: bool,
) -> Result<()> {
    if ensure_exist {
        let dir = filepath.as_ref().parent().ok_or_eyre(format!(
            "Failed to get parent of `{}`.",
            filepath.as_ref().display()
        ))?;
        std::fs::create_dir_all(dir).context(format!(
            "Failed to create directory `{}` recursively.",
            dir.display()
        ))?;
    }
    std::fs::write(filepath.as_ref(), &content).context(format!(
        "Failed to write `{}`.",
        filepath.as_ref().display()
    ))?;
    Ok(())
}

pub(crate) fn open(filepath: impl AsRef<Path>) -> Result<std::fs::File> {
    std::fs::File::open(filepath.as_ref()).context(format!(
        "Failed to open file `{}`.",
        filepath.as_ref().display()
    ))
}

pub(crate) fn create(filepath: impl AsRef<Path>) -> Result<std::fs::File> {
    std::fs::File::create(filepath.as_ref()).context(format!(
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
        .context(format!("Failed to read `{}`.", filepath.as_ref().display()))?;
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
            .context(format!("Failed to create `{}` recursively.", dir.display()))?;
    }
    tokio::fs::write(filepath.as_ref(), &content)
        .await
        .context(format!(
            "Failed to write `{}`.",
            filepath.as_ref().display()
        ))?;
    Ok(())
}

pub(crate) async fn create_async(filepath: impl AsRef<Path>) -> Result<tokio::fs::File> {
    tokio::fs::File::create(filepath.as_ref())
        .await
        .context(format!(
            "Failed to create file `{}`.",
            filepath.as_ref().display()
        ))
}

pub(crate) async fn open_async(filepath: impl AsRef<Path>) -> Result<tokio::fs::File> {
    tokio::fs::File::open(filepath.as_ref())
        .await
        .context(format!(
            "Failed to open file `{}`.",
            filepath.as_ref().display()
        ))
}
