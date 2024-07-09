use crate::config::compile_opts;
use std::path::PathBuf;
use ::{anyhow::Result, tokio::process::Command};

#[derive(Debug)]
pub(crate) enum CompileMode {
    Debug,
    Release,
    Expand,
}

#[derive(Debug)]
pub(crate) struct CompileCommand {
    pub compiler: String,
    pub include_dirs: Vec<PathBuf>,
    pub macros: Vec<String>,
    pub opts: Vec<String>,
    pub src: Option<PathBuf>,
    pub dst: Option<PathBuf>,
}
impl CompileCommand {
    pub fn load_config(mode: CompileMode) -> Result<Self> {
        let loaded_opts = compile_opts::load_opts()?;
        let mut opts = loaded_opts.common_opts.clone();
        match mode {
            CompileMode::Debug => opts.extend(loaded_opts.debug_opts.clone()),
            CompileMode::Release => opts.extend(loaded_opts.release_opts.clone()),
            CompileMode::Expand => {
                opts = ["-E", "-P", "-nostdinc++", "-nostdinc"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
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
            src: None,
            dst: None,
        })
    }
    pub async fn exec_compile(&self) -> Result<String> {
        let mut args = self.opts.clone();
        for dir in &self.include_dirs {
            args.push("-I".to_string());
            args.push(dir.to_string_lossy().into_owned());
        }
        for macro_ in &self.macros {
            args.push("-D".to_string());
            args.push(macro_.to_string());
        }
        if let Some(dst) = &self.dst {
            args.push("-o".to_string());
            args.push(dst.to_string_lossy().into_owned());
        }
        if let Some(src) = &self.src {
            args.push(src.to_string_lossy().into_owned());
        }
        log::info!("{} {}", self.compiler, args.join(" "));
        let output = Command::new(&self.compiler).args(&args).output().await?;
        let stderr = String::from_utf8(output.stderr)?;
        if !stderr.is_empty() {
            log::error!("{}", stderr);
            Err(anyhow::anyhow!("Compile error"))
        } else {
            Ok(String::from_utf8(output.stdout)?)
        }
    }
}
