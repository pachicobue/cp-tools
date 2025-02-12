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

    use cpt_stdx::path::{get_extension, get_filestem};

    let mut cases: Vec<Testcase> = Vec::new();
    let mut builder = WalkBuilder::new(dir);
    builder.standard_filters(false).max_depth(Some(1));
    for entry in builder.build().filter(|entry| entry.as_ref().is_ok()) {
        let input_path = entry.unwrap().path().to_path_buf();
        if get_extension(&input_path) == INPUT_EXT {
            let output_path = input_path.with_extension(OUTPUT_EXT);
            cases.push(Testcase {
                casename: get_filestem(&input_path),
                input: input_path,
                output: output_path.exists().then_some(output_path),
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
