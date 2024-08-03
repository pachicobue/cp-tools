mod atcoder;
mod base_client;

use crate::oj_client::atcoder::{AtcoderClient, ATCODER_HOST};
use ::{
    color_eyre::eyre::{OptionExt, Result},
    url::Url,
};

trait OjClient: Sized {
    fn new() -> Result<Self>;
    fn login(&self) -> Result<()>;
    fn get_problem_info(&self, url: &Url) -> Result<ProblemInfo>;
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

fn login_gen<O: OjClient>() -> Result<()> {
    O::new()?.login()
}

fn get_problem_info_gen<O: OjClient>(url: &Url) -> Result<ProblemInfo> {
    O::new()?.get_problem_info(url)
}

pub(crate) fn login(url: &Url) -> Result<()> {
    match url.host_str().ok_or_eyre("Invalid URL")? {
        ATCODER_HOST => login_gen::<AtcoderClient>(),
        _ => unimplemented!(),
    }
}

pub(crate) fn get_problem_info(url: &Url) -> Result<ProblemInfo> {
    match url.host_str().ok_or_eyre("Invalid URL")? {
        ATCODER_HOST => get_problem_info_gen::<AtcoderClient>(url),
        _ => unimplemented!(),
    }
}
