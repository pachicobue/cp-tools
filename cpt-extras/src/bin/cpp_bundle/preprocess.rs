use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub(super) enum Error {
    #[error("Preprocess failed.")]
    PreprocessFailed(#[source] cpt_stdx::process::Error),
}

pub(super) fn expand(
    cxx: &str,
    src: &Path,
    cxx_args: &Vec<String>,
    with_comment: bool,
    deps: crate::isystem::SystemDependencies,
) -> Result<String, Error> {
    use std::process::Stdio;

    use cpt_stdx::process::{Command, IoRedirection};

    let output = cpt_stdx::tempfile::with_tempdir(|tempdir| -> Result<String, Error> {
        create_dummy_headers(tempdir.path(), deps.system_headers);

        let args = [
            vec![
                "-E".to_owned(),
                "-P".to_owned(),
                "-nostdinc".to_owned(),
                "-nostdinc++".to_owned(),
            ],
            if with_comment {
                vec!["-C".to_owned()]
            } else {
                vec![]
            },
            vec![format!("{}", src.display())],
            cxx_args.to_owned(),
            vec!["-I".to_owned(), format!("{}", tempdir.path().display())],
        ]
        .concat();
        let command = Command::new(cxx, args);
        log::debug!("$ {}", command);

        const TIMELIMIT_MS: u64 = 3000;
        let res = command
            .exec(
                IoRedirection {
                    stdin: Stdio::null(),
                    stdout: Stdio::piped(),
                    stderr: Stdio::piped(),
                },
                TIMELIMIT_MS,
                true,
            )
            .map_err(Error::PreprocessFailed)?;
        Ok(deps
            .system_header_tops
            .iter()
            .fold("".to_owned(), |s, path| {
                format!("{}\n#include <{}>", s, path.display())
            })
            + "\n"
            + &res.detail.stdout)
    })?;
    Ok(output)
}

fn create_dummy_headers<I>(tempdir: &Path, sys_header_deps: I)
where
    I: IntoIterator<Item = PathBuf>,
{
    for header in sys_header_deps {
        cpt_stdx::fs::write(tempdir.join(&header), "", true).unwrap();
    }
}
