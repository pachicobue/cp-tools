use crate::oj_client;
use ::{anyhow::Result, clap::Args, reqwest::Url};

#[derive(Args, Debug)]
pub(crate) struct LoginArgs {
    /// URL
    #[arg(required = true)]
    url: Url,
}

pub(crate) async fn login(args: &LoginArgs) -> Result<()> {
    log::info!("Logging in to {}...", args.url);
    check_args(args)?;
    oj_client::login(&args.url).await?;
    Ok(())
}

fn check_args(_args: &LoginArgs) -> Result<()> {
    Ok(())
}
