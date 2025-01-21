use std::path::{Path, PathBuf};

const INPUT_EXT: &str = "in";
const OUTPUT_EXT: &str = "out";
const HACKCASE_PREFIX: &str = "Generated_";

#[derive(Debug, Clone)]
pub(crate) struct Testcase {
    pub(crate) casename: String,
    pub(crate) input: PathBuf,
    pub(crate) output: Option<PathBuf>,
}

pub(crate) fn collect(dir: &Path) -> Vec<Testcase> {
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

pub(crate) fn new_hackcase(dir: &Path) -> Testcase {
    use std::collections::BTreeSet;
    let mut hackcases = BTreeSet::<String>::new();
    for cases in collect(dir) {
        if let Some(hackcase) = cases.casename.strip_prefix(HACKCASE_PREFIX) {
            hackcases.insert(hackcase.to_owned());
        }
    }
    let mut no = 0;
    loop {
        if !hackcases.contains(&no.to_string()) {
            let casename = format!("{}{}", HACKCASE_PREFIX, no);
            return Testcase {
                casename: casename.to_owned(),
                input: dir.to_owned().join(casename.to_owned() + "." + INPUT_EXT),
                output: Some(dir.to_owned().join(casename + "." + OUTPUT_EXT)),
            };
        }
        no += 1;
    }
}
