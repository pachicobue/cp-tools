use std::path::{Path, PathBuf};

const INPUT_EXT: &str = "in";
const OUTPUT_EXT: &str = "out";

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Verdict {
    Ac,
    Wa,
    Re,
    Tle,
    Ie,
}

impl std::fmt::Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Verdict::Ac => "AC",
            Verdict::Wa => "WA",
            Verdict::Re => "RE",
            Verdict::Tle => "TLE",
            Verdict::Ie => "IE",
        };
        str.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct JudgePaths {
    pub(crate) input: [PathBuf; 2],
    pub(crate) expect: Option<PathBuf>,
    pub(crate) actual: [PathBuf; 2],
}

pub(crate) fn collect_judge_paths(dir: &Path, tempdir: &Path) -> Vec<JudgePaths> {
    use ignore::WalkBuilder;

    use cpt_stdx::path::PathInfo;

    let mut cases: Vec<JudgePaths> = Vec::new();
    let mut builder = WalkBuilder::new(dir);
    builder.standard_filters(false).max_depth(Some(1));

    for entry in builder.build().filter(|entry| {
        entry
            .as_ref()
            .is_ok_and(|entry| entry.path().extension().unwrap_or_default() == INPUT_EXT)
    }) {
        let input_pathinfo = PathInfo::from(entry.unwrap().path());
        let input = input_pathinfo.path;
        let output = PathInfo::new(
            &input_pathinfo.basedir,
            &input_pathinfo.filestem,
            OUTPUT_EXT,
        )
        .path;
        let testname = input_pathinfo.filestem;
        cases.push(JudgePaths {
            input: [
                input.clone(),
                tempdir.join(format!("{}_second.in", testname)),
            ],
            expect: if output.exists() { Some(output) } else { None },
            actual: [
                tempdir.join(format!("{}_first.actual", testname)),
                tempdir.join(format!("{}_second.actual", testname)),
            ],
        });
    }
    cases
}
