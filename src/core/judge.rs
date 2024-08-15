use std::{
    fmt,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

const INPUT_EXT: &str = "in";
const OUTPUT_EXT: &str = "out";

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Verdict {
    Ac,
    Wa,
    Re,
    Tle,
    Ce,
    Ie,
}
impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Verdict::Ac => "AC",
            Verdict::Wa => "WA",
            Verdict::Re => "RE",
            Verdict::Tle => "TLE",
            Verdict::Ce => "CE",
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
    let mut cases: Vec<JudgePaths> = Vec::new();
    for entry in WalkDir::new(dir).max_depth(1).into_iter().filter(|entry| {
        entry
            .as_ref()
            .is_ok_and(|entry| entry.path().extension().unwrap_or_default() == INPUT_EXT)
    }) {
        let input = entry.unwrap().path().to_path_buf();
        let output = input.clone().with_extension(OUTPUT_EXT);
        let testname = input.file_stem().unwrap();
        cases.push(JudgePaths {
            input: [
                input.clone(),
                tempdir.join(&format!("{}_second.in", testname.to_string_lossy())),
            ],
            expect: if output.exists() { Some(output) } else { None },
            actual: [
                tempdir.join(&format!("{}_first.actual", testname.to_string_lossy())),
                tempdir.join(&format!("{}_second.actual", testname.to_string_lossy())),
            ],
        });
    }
    cases
}
