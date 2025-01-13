use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub(super) enum Error {
    #[error("Fetch isystem search path failed.")]
    SearchIsystemPathFailed(#[source] cpt_stdx::process::Error),
    #[error("Get dependency failed.")]
    GetDependencyFailed(#[source] cpt_stdx::process::Error),
}

#[derive(Debug, Default)]
pub(super) struct SystemDependencies {
    pub(super) system_headers: BTreeSet<PathBuf>,
    pub(super) system_header_tops: BTreeSet<PathBuf>,
    pub(super) library_headers: BTreeSet<PathBuf>,
}

pub(super) fn get_isys_deps(
    cxx: &str,
    src: &Path,
    cxx_args: &Vec<String>,
) -> Result<SystemDependencies, Error> {
    isys_deps(cxx, src, cxx_args, &isystem_paths(cxx)?)
}

fn isystem_paths(cxx: &str) -> Result<Vec<PathBuf>, Error> {
    use std::process::Stdio;

    use cpt_stdx::process::{Command, IoRedirection};

    let args = vec!["-E", "-xc++", "-v", "-"];
    let command = Command::new(cxx, args);
    log::debug!("$ {}", command);

    let res = command
        .exec(
            IoRedirection {
                stdin: Stdio::null(),
                stdout: Stdio::piped(),
                stderr: Stdio::piped(),
            },
            3000,
            true,
        )
        .map_err(Error::SearchIsystemPathFailed)?;
    let include_paths: Vec<PathBuf> = res
        .detail
        .stderr
        .lines()
        .skip_while(|line| !line.starts_with("#include <...> search starts here:"))
        .take_while(|line| !line.starts_with("End of search list."))
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| PathBuf::from(line.trim()))
        .collect();
    log::trace!("system_include_paths: {:#?}", include_paths);
    Ok(include_paths)
}

fn isys_deps(
    cxx: &str,
    src: &Path,
    cxx_args: &Vec<String>,
    isystem_paths: &[PathBuf],
) -> Result<SystemDependencies, Error> {
    use std::process::Stdio;

    use cpt_stdx::process::{Command, IoRedirection};

    let args = [
        vec![
            "-E".to_owned(),
            "-H".to_owned(),
            format!("{}", src.display()),
        ],
        cxx_args.to_owned(),
    ]
    .concat();
    let command = Command::new(cxx, args);
    log::debug!("$ {}", command);

    let res = command
        .exec(
            IoRedirection {
                stdin: Stdio::null(),
                stdout: Stdio::piped(),
                stderr: Stdio::piped(),
            },
            3000,
            true,
        )
        .map_err(Error::GetDependencyFailed)?;
    let mut deps = SystemDependencies::default();
    {
        let re = regex::Regex::new(r"(?m)^(\.+) (.*)$").unwrap();
        let mut stack = Vec::<(usize, bool)>::new();
        for (_, [dots, path]) in re.captures_iter(&res.detail.stderr).map(|c| c.extract()) {
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

            if let Some(path) = shorten_path(&path, isystem_paths) {
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

fn shorten_path(full_path: &Path, inc_dirs: &[PathBuf]) -> Option<PathBuf> {
    inc_dirs
        .iter()
        .filter_map(|dir| full_path.strip_prefix(dir).ok().map(|rel| rel.to_owned()))
        .min_by_key(|rel| rel.to_string_lossy().len())
}
