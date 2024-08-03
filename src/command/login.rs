use crate::oj_client;
use ::{
    clap::{Args, ValueHint},
    color_eyre::eyre::Result,
    url::Url,
};

#[derive(Args, Debug)]
pub(crate) struct LoginArgs {
    /// URL
    #[arg(required = true ,value_hint = ValueHint::Url)]
    url: Url,
}

pub(crate) fn login(args: &LoginArgs) -> Result<()> {
    log::info!("Logging in to {}...", args.url);
    check_args(args)?;
    oj_client::login(&args.url)?;
    Ok(())
}

fn check_args(_args: &LoginArgs) -> Result<()> {
    Ok(())
}
