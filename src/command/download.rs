use crate::{config, judge::TestCase, oj_client};
use std::{fs, path::PathBuf};
use ::{
    clap::{Args, ValueHint},
    color_eyre::eyre::Result,
    console::style,
    url::Url,
};

#[derive(Args, Debug)]
pub(crate) struct DownloadArgs {
    /// URL
    #[arg(required = true, value_hint = ValueHint::Url)]
    url: Url,

    /// 出力先ディレクトリ
    #[arg(short = 'd', long, value_hint = ValueHint::FilePath)]
    dir: Option<PathBuf>,
}

pub(crate) fn download(args: &DownloadArgs) -> Result<()> {
    log::info!("{}\n{:?}", style("Download sample").bold().green(), args);
    check_args(args)?;

    let dir = match &args.dir {
        Some(dir) => dir.clone(),
        None => default_dir(),
    };
    // log::trace!("Create directory {}", dir.to_string_lossy());
    // fs::remove_dir_all(&dir)?;
    // fs::create_dir_all(&dir)?;

    let info = oj_client::get_problem_info(&args.url)?;
    for i in 0..info.samples.case_num {
        let name = format!("Sample{:03}", i);
        let cases = TestCase::from_name(&dir, &name);
        log::debug!("Save input -> {}", cases.input.to_string_lossy());
        log::trace!("Input: \n{}", info.samples.inputs[i]);
        // fs::write(&cases.input, &info.samples.inputs[i])?;

        log::debug!("Save output -> {}", cases.output.to_string_lossy());
        log::trace!("Output: \n{}", info.samples.outputs[i]);
        // fs::write(&cases.output, &info.samples.outputs[i])?;
    }
    log::info!("{}", style("Download completed").bold().green(),);
    Ok(())
}

fn check_args(_args: &DownloadArgs) -> Result<()> {
    Ok(())
}

fn default_dir() -> PathBuf {
    config::dirs::workspace_dir().join("TestCases")
}
