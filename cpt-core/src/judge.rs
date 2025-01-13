pub mod batch;

use std::path::{Path, PathBuf};

const INPUT_EXT: &str = "in";
const OUTPUT_EXT: &str = "out";

#[derive(Debug, Clone, strum::Display, strum::EnumCount, strum::EnumIter)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Verdict {
    Ac,
    Wa,
    Re,
    Tle,
}

#[derive(Debug, Clone)]
pub(crate) struct Testcase {
    pub(crate) casename: String,
    pub(crate) input: PathBuf,
    pub(crate) output: Option<PathBuf>,
}

pub(crate) fn collect_cases(dir: &Path) -> Vec<Testcase> {
    use ignore::WalkBuilder;

    use cpt_stdx::path::PathInfo;

    let mut cases: Vec<Testcase> = Vec::new();
    let mut builder = WalkBuilder::new(dir);
    builder.standard_filters(false).max_depth(Some(1));

    for entry in builder.build().filter(|entry| entry.as_ref().is_ok()) {
        let input_pathinfo = PathInfo::from(entry.unwrap().path());
        if input_pathinfo.extension == INPUT_EXT {
            let input = input_pathinfo.path;
            let output = PathInfo::new(
                &input_pathinfo.basedir,
                &input_pathinfo.filestem,
                OUTPUT_EXT,
            )
            .path;
            cases.push(Testcase {
                casename: input_pathinfo.filestem,
                input,
                output: output.exists().then_some(output),
            });
        }
    }
    cases
}
