use std::path::{Path, PathBuf};
use ::{anyhow::Result, reqwest::Url};

const INPUT_EXT: &str = "in";
const OUTPUT_EXT: &str = "out";

pub(crate) struct TestCase {
    pub input: PathBuf,
    pub output: PathBuf,
}
impl TestCase {
    pub fn new(input: &Path, output: &Path) -> Self {
        Self {
            input: input.to_path_buf(),
            output: output.to_path_buf(),
        }
    }
    pub fn from_name(dir: &Path, name: &str) -> Self {
        Self::new(
            &dir.join(name.to_string() + "." + INPUT_EXT),
            &dir.join(name.to_string() + "." + OUTPUT_EXT),
        )
    }
    pub fn is_input_only(&self) -> bool {
        self.output.exists()
    }
    pub fn collect(dir: &Path) -> Result<Vec<Self>> {
        assert!(dir.is_dir());
        Ok(dir
            .read_dir()?
            .filter_map(Result::ok)
            .filter(|f| f.path().ends_with(INPUT_EXT))
            .map(|f| TestCase::from_name(dir, f.file_name().to_str().unwrap()))
            .collect())
    }
}

pub(crate) enum JudgeType {
    Batch,
}

pub(crate) struct ProblemInfo {
    pub url: Url,
    pub test_cases: Vec<TestCase>,
}

pub(crate) struct Judge {
    test_cases: Vec<TestCase>,
}
