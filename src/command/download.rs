use crate::{config, judge::TestCase, oj_client};
use std::{fs, path::PathBuf};
use ::{anyhow::Result, clap::Args, reqwest::Url};

#[derive(Args, Debug)]
pub(crate) struct DownloadArgs {
    /// URL
    #[arg(required = true)]
    url: Url,

    /// 出力先ディレクトリ
    #[arg(short = 'd', long)]
    dir: Option<String>,
}

pub(crate) async fn download(args: &DownloadArgs) -> Result<()> {
    log::info!("Downloading sample from {} ...", args.url);
    check_args(args)?;

    let dir = match &args.dir {
        Some(dir) => PathBuf::from(dir),
        None => default_dir()?,
    };
    log::trace!("Create directory {}", dir.to_string_lossy());
    fs::remove_dir_all(&dir)?;
    fs::create_dir_all(&dir)?;

    let info = oj_client::get_problem_info(&args.url).await?;
    for i in 0..info.samples.case_num {
        let name = format!("Sample{:03}", i);
        let cases = TestCase::from_name(&dir, &name);
        log::debug!("Save input -> {}", cases.input.to_string_lossy());
        log::trace!("Input: \n{}", info.samples.inputs[i]);
        fs::write(&cases.input, &info.samples.inputs[i])?;

        log::debug!("Save output -> {}", cases.output.to_string_lossy());
        log::trace!("Output: \n{}", info.samples.outputs[i]);
        fs::write(&cases.output, &info.samples.outputs[i])?;
    }
    Ok(())
}

fn check_args(_args: &DownloadArgs) -> Result<()> {
    Ok(())
}

fn default_dir() -> Result<PathBuf> {
    let dir = config::dirs::workspace_dir().join("TestCases");
    Ok(dir)
}
