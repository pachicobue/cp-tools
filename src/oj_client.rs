mod atcoder;
mod base_client;

use crate::oj_client::atcoder::{AtcoderClient, ATCODER_HOST};
use ::{
    anyhow::{Context as _, Result},
    reqwest::Url,
};

trait OjClient: Sized {
    fn new() -> Result<Self>;
    async fn login(&self) -> Result<()>;
    async fn get_problem_info(&self, url: &Url) -> Result<ProblemInfo>;
}

#[derive(Debug)]
pub(crate) struct ProblemInfo {
    pub samples: SampleInfo,
}

#[derive(Debug)]
pub(crate) struct SampleInfo {
    pub case_num: usize,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

async fn login_gen<O: OjClient>() -> Result<()> {
    O::new()?.login().await
}

async fn get_problem_info_gen<O: OjClient>(url: &Url) -> Result<ProblemInfo> {
    O::new()?.get_problem_info(url).await
}

pub(crate) async fn login(url: &Url) -> Result<()> {
    match url.host_str().context("Invalid URL")? {
        ATCODER_HOST => login_gen::<AtcoderClient>().await,
        _ => unimplemented!(),
    }
}

pub(crate) async fn get_problem_info(url: &Url) -> Result<ProblemInfo> {
    match url.host_str().context("Invalid URL")? {
        ATCODER_HOST => get_problem_info_gen::<AtcoderClient>(url).await,
        _ => unimplemented!(),
    }
}
