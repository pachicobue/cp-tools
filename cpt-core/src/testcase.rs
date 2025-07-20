use std::path::{Path, PathBuf};

const INPUT_EXT: &str = "in";
const OUTPUT_EXT: &str = "out";
const HACKCASE_PREFIX: &str = "Generated_";

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Failed to copy testcase files.")]
    Copy(#[from] cpt_stdx::fs::Error),
}

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

pub(crate) fn new_hackcase(dir: &Path) -> (Testcase, Testcase) {
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
            let temp_dir = std::env::temp_dir();
            let temp_case = Testcase {
                casename: casename.clone(),
                input: temp_dir.join(casename.clone() + "." + INPUT_EXT),
                output: Some(temp_dir.join(casename.clone() + "." + OUTPUT_EXT)),
            };
            let final_case = Testcase {
                casename,
                input: dir
                    .to_owned()
                    .join(temp_case.casename.clone() + "." + INPUT_EXT),
                output: Some(
                    dir.to_owned()
                        .join(temp_case.casename.clone() + "." + OUTPUT_EXT),
                ),
            };
            return (temp_case, final_case);
        }
        no += 1;
    }
}

impl Testcase {
    pub(crate) fn copy_to(&self, target: &Testcase) -> Result<(), Error> {
        cpt_stdx::fs::copy(&self.input, &target.input)?;
        if let (Some(src_output), Some(target_output)) = (&self.output, &target.output) {
            cpt_stdx::fs::copy(src_output, target_output)?;
        }
        Ok(())
    }

    pub(crate) fn copy_to_with_intermediate_files(
        &self,
        target: &Testcase,
        temp_dir: &std::path::Path,
        final_dir: &std::path::Path,
    ) -> Result<(), Error> {
        // Copy main testcase files
        self.copy_to(target)?;

        // Copy intermediate files if they exist
        let intermediate_extensions = ["actual.txt", "debug.txt", "judge.txt"];
        for ext in &intermediate_extensions {
            let temp_file = temp_dir.join(format!("{}.{}", self.casename, ext));
            let final_file = final_dir.join(format!("{}.{}", self.casename, ext));

            if temp_file.exists() {
                if let Err(_) = cpt_stdx::fs::copy(&temp_file, &final_file) {
                    // Intermediate files are optional, so we don't fail if copy fails
                    log::warn!("Failed to copy intermediate file: {}", temp_file.display());
                }
            }
        }
        Ok(())
    }
}
