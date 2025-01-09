use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use thiserror::Error;

use cpt_stdx::process::{
    command_task, CommandExpression, CommandIoRedirection, CommandResultSummary,
};
use cpt_stdx::task::run_task;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("Failed to get system include paths:\n{0}")]
    FailedToGetSystemIncludePaths(String),
    #[error("Failed to get dependency:\n{0}")]
    FailedToGetDependency(String),
}

#[derive(Debug, Default)]
pub(crate) struct SystemDependencies {
    pub system_headers: BTreeSet<PathBuf>,
    pub system_header_tops: BTreeSet<PathBuf>,
    pub library_headers: BTreeSet<PathBuf>,
}

pub(crate) fn get_system_header_deps(
    cxx: &str,
    src: &Path,
    inc_dirs: &Vec<PathBuf>,
    macros: &Vec<String>,
    std: &Option<String>,
) -> Result<SystemDependencies, Error> {
    system_header_deps(
        cxx,
        src,
        inc_dirs,
        macros,
        std,
        &system_header_search_paths(cxx, macros, std)?,
    )
}

fn system_header_search_paths(
    cxx: &str,
    macros: &Vec<String>,
    std: &Option<String>,
) -> Result<Vec<PathBuf>, Error> {
    let mut args = vec!["-E".to_owned(), "-xc++".to_owned(), "-v".to_owned()];
    for mac in macros {
        args.push(format!("-D{}", mac));
    }
    if std.is_some() {
        args.push(format!("-std={}", std.as_ref().unwrap()));
    }
    args.push("-".to_owned());
    let command = CommandExpression::new(cxx, args);
    log::debug!("$ {}", command);

    const TL: f32 = 10.;
    let res = run_task(command_task(
        command,
        CommandIoRedirection::piped(),
        Some(TL),
    ))
    .map_err(|e| Error::FailedToGetDependency(e.to_string()))?;
    let stderr = match res.summary {
        CommandResultSummary::Success => res.detail.stderr,
        CommandResultSummary::Aborted => {
            return Err(Error::FailedToGetSystemIncludePaths(res.detail.stderr));
        }
        CommandResultSummary::Timeout => {
            return Err(Error::FailedToGetSystemIncludePaths(format!(
                "Timeout {}s",
                TL
            )));
        }
    };
    let mut include_paths = vec![];
    let mut collecting = false;
    for line in stderr.lines() {
        if line.starts_with("#include <...> search starts here:") {
            collecting = true;
            continue;
        } else if line.starts_with("End of search list.") {
            break;
        }
        if collecting {
            let path = line.trim();
            if !path.is_empty() {
                include_paths.push(PathBuf::from(path));
            }
        }
    }
    log::trace!("system_include_paths: {:#?}", include_paths);
    Ok(include_paths)
}

fn system_header_deps(
    cxx: &str,
    src: &Path,
    inc_dirs: &Vec<PathBuf>,
    macros: &Vec<String>,
    std: &Option<String>,
    system_include_dirs: &Vec<PathBuf>,
) -> Result<SystemDependencies, Error> {
    let mut args = vec![
        "-E".to_owned(),
        "-H".to_owned(),
        format!("{}", src.display()),
    ];
    for dir in inc_dirs {
        args.push(format!("-I{}", dir.display()));
    }
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
    .map_err(|e| Error::FailedToGetDependency(e.to_string()))?;
    let stderr = match res.summary {
        CommandResultSummary::Success => res.detail.stderr,
        CommandResultSummary::Aborted => {
            return Err(Error::FailedToGetDependency(res.detail.stderr));
        }
        CommandResultSummary::Timeout => {
            return Err(Error::FailedToGetDependency(format!("Timeout {}s", TL)));
        }
    };

    let mut deps = SystemDependencies::default();
    {
        let re = regex::Regex::new(r"(?m)^(\.+) (.*)$").unwrap();
        let mut stack = Vec::<(usize, bool)>::new();
        for (_, [dots, path]) in re.captures_iter(&stderr).map(|c| c.extract()) {
            log::debug!("{} {}", dots, path);
            let depth = dots.len();
            let path = PathBuf::from(path);
            let mut hide = false;
            while let Some(&(pdepth, psys)) = stack.last() {
                if pdepth < depth {
                    hide = psys;
                    break;
                }
                stack.pop().unwrap();
            }

            if let Some(path) = shorten_path(&path, &system_include_dirs) {
                deps.system_headers.insert(path.to_owned());
                if !hide {
                    deps.system_header_tops.insert(path);
                }
                stack.push((depth, true));
            } else {
                deps.library_headers.insert(path);
                stack.push((depth, false));
            }
        }
    }
    Ok(deps)
}

fn shorten_path(full_path: &Path, inc_dirs: &Vec<PathBuf>) -> Option<PathBuf> {
    let mut found = false;
    let mut short = full_path.to_owned();
    for dir in inc_dirs {
        if let Ok(rel) = full_path.strip_prefix(dir) {
            found = true;
            if short.to_string_lossy().len() > rel.to_string_lossy().len() {
                short = rel.to_owned();
            }
        }
    }
    found.then_some(short)
}
