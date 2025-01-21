#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Failed to execute input_generator.")]
    InputGeneration(#[source] cpt_stdx::process::Error),
    #[error("Failed to execute output_generator.")]
    OutputGeneration(#[source] cpt_stdx::process::Error),
}

pub(crate) fn generate(
    testcase: &crate::testcase::Testcase,
    input_generator_command: &str,
    output_generator_command: &Option<String>,
    timelimit_ms: u64,
) -> Result<crate::testcase::Testcase, Error> {
    use std::process::Stdio;

    use cpt_stdx::fs;
    use cpt_stdx::process::{Command, IoRedirection};

    log::info!("[Generator][{}] Start", testcase.casename);
    let mut case = testcase.to_owned();
    Command::from(input_generator_command)
        .exec(
            IoRedirection {
                stdin: Stdio::null(),
                stdout: Stdio::from(fs::create(&testcase.input, true).unwrap()),
                stderr: Stdio::piped(),
            },
            timelimit_ms,
            true,
        )
        .map_err(Error::InputGeneration)?;
    if let Some(output_generator_command) = output_generator_command {
        Command::from(output_generator_command)
            .exec(
                IoRedirection {
                    stdin: Stdio::from(fs::open(&testcase.input).unwrap()),
                    stdout: Stdio::from(
                        fs::create(testcase.output.as_ref().unwrap(), true).unwrap(),
                    ),
                    stderr: Stdio::piped(),
                },
                timelimit_ms,
                true,
            )
            .map_err(Error::OutputGeneration)?;
    } else {
        case.output = None;
    }
    log::info!("[Generator][{}] End", testcase.casename);
    Ok(case)
}
