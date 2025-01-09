use std::path::{Path, PathBuf};

use cpt_stdx::tempfile::with_tempdir;
use thiserror::Error;

use cpt_stdx::fs;
use cpt_stdx::process::{
    command_task, CommandExpression, CommandIoRedirection, CommandResultSummary,
};
use cpt_stdx::task::run_task;

use crate::isystem::SystemDependencies;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to get create dummy header:\n{0}")]
    FailedToCreateDummyHeaders(fs::FilesystemError),
    #[error("Failed to preprocess:\n{0}")]
    FailedToPreprocess(String),
}

pub(crate) fn expand(
    cxx: &str,
    src: &Path,
    inc_dirs: &Vec<PathBuf>,
    macros: &Vec<String>,
    std: &Option<String>,
    with_comment: bool,
    deps: SystemDependencies,
) -> Result<String, Error> {
    let output = with_tempdir(|tempdir| {
        create_dummy_headers(tempdir.path(), deps.system_headers)?;

        let mut args = vec![
            "-E".to_owned(),
            "-P".to_owned(),
            "-nostdinc++".to_owned(),
            "-nostdinc".to_owned(),
        ];
        if with_comment {
            args.push("-C".to_owned());
        }
        args.push(format!("{}", src.display()));
        for dir in inc_dirs {
            args.push(format!("-I{}", dir.display()));
        }
        args.push(format!("-I{}", tempdir.path().display()));
        for mac in macros {
            args.push(format!("-D{}", mac));
        }
        if std.is_some() {
            args.push(format!("-std={}", std.as_ref().unwrap()));
        }
        let command = CommandExpression::new(cxx, args);
        log::debug!("$ {}", command);

        const TL: f32 = 10.;
        let res = run_task(command_task(
            command,
            CommandIoRedirection::piped(),
            Some(TL),
        ))
        .map_err(|e| Error::FailedToPreprocess(e.to_string()))?;
        let out = match res.summary {
            CommandResultSummary::Success => res.detail.stdout,
            CommandResultSummary::Aborted => {
                return Err(Error::FailedToPreprocess(res.detail.stderr));
            }
            CommandResultSummary::Timeout => {
                return Err(Error::FailedToPreprocess(format!("Timeout {}s", TL)));
            }
        };
        Ok(deps
            .system_header_tops
            .iter()
            .fold("".to_owned(), |s, path| {
                format!("{}\n#include <{}>", s, path.display())
            })
            + "\n"
            + &out)
    })?;
    Ok(output)
}

fn create_dummy_headers<I>(tempdir: &Path, sys_header_deps: I) -> Result<(), Error>
where
    I: IntoIterator<Item = PathBuf>,
{
    for header in sys_header_deps {
        fs::write(
            tempdir.join(&header),
            "",
            // format!("#INCLUDE <{}>", header.display()),
            true,
        )
        .map_err(Error::FailedToCreateDummyHeaders)?;
    }
    Ok(())
}
