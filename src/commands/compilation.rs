use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;

use super::compile_opts;
use crate::core::process::{run_command_simple, CommandExpression};

#[derive(Debug)]
pub(crate) enum CompileMode {
    Debug,
    Release,
    Expand,
}

#[derive(Debug)]
pub(crate) struct CompileCommand {
    compiler: String,
    pub(crate) include_dirs: Vec<PathBuf>,
    macros: Vec<String>,
    opts: Vec<String>,
}
impl CompileCommand {
    pub fn load_config(mode: CompileMode) -> Result<Self> {
        let loaded_opts = compile_opts::load_opts()?;
        let mut opts = loaded_opts.common_opts.clone();
        match mode {
            CompileMode::Debug => opts.extend(loaded_opts.debug_opts.clone()),
            CompileMode::Release => opts.extend(loaded_opts.release_opts.clone()),
            CompileMode::Expand => {
                opts = ["-E"].iter().map(|s| s.to_string()).collect();
            }
        }
        let macros = match mode {
            CompileMode::Debug => loaded_opts.macros.clone(),
            CompileMode::Release => loaded_opts.macros.clone(),
            CompileMode::Expand => [].to_vec(),
        };
        Ok(Self {
            compiler: loaded_opts.compiler.clone(),
            include_dirs: loaded_opts
                .include_directories
                .iter()
                .map(PathBuf::from)
                .collect(),
            macros,
            opts,
        })
    }

    pub fn exec_compilation(&self, src: &Path, dst: Option<&Path>) -> Result<String> {
        let mut args = self.opts.clone();
        for dir in &self.include_dirs {
            args.push("-I".to_string());
            args.push(dir.to_string_lossy().into_owned());
        }
        for macro_ in &self.macros {
            args.push("-D".to_string());
            args.push(macro_.to_string());
        }
        if let Some(dst) = dst {
            args.push("-o".to_string());
            args.push(dst.to_string_lossy().into_owned());
        }
        args.push(src.to_string_lossy().into_owned());

        let output = run_command_simple(CommandExpression::new(&self.compiler, args))?
            .detail_of_success()?
            .stdout;
        Ok(output)
    }
}
