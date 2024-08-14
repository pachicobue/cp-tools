use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;
use itertools::Itertools;

use crate::process::{run_multiple, CmdExpression, CmdIoRedirection};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Verdict {
    Ac,
    Wa,
    Re,
    Tle,
    Ie,

    Wj,
}

#[derive(Debug, Clone)]
pub(crate) struct Testcase {
    pub(crate) input: PathBuf,
    pub(crate) expect: Option<PathBuf>,
    pub(crate) actual: Option<PathBuf>,
}
