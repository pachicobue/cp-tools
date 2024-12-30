use std::{
    ffi::{OsStr, OsString},
    process::Stdio,
    time::Duration,
};

use itertools::Itertools;
use strum;
use tokio;

use crate::core::{printer::abbr, task::run_task};

/// コマンドの実行結果を表す列挙型
#[derive(Debug, Clone, strum::EnumIs)]
pub(crate) enum CommandResult {
    Success(CommandResultDetail),
    Aborted(CommandResultDetail),
    Timeout(CommandResultDetail),
}
impl CommandResult {
    /// コマンドの実行結果の詳細を取得する関数
    ///
    /// # 戻り値
    ///
    /// コマンドの実行結果の詳細
    pub(crate) fn get_detail(&self) -> &CommandResultDetail {
        match self {
            Self::Success(detail) => detail,
            Self::Aborted(detail) => detail,
            Self::Timeout(detail) => detail,
        }
    }
}

/// コマンドの実行結果の詳細を格納する構造体
#[derive(Debug, Clone)]
pub(crate) struct CommandResultDetail {
    pub stdout: String,
    pub stderr: String,
    pub elapsed: Duration,
}

/// コマンドの実行設定を格納する構造体
#[derive(Debug, Clone)]
pub(crate) struct CommandExpression {
    pub program: OsString,
    pub args: Vec<OsString>,
}
impl CommandExpression {
    /// コマンドの実行設定を生成する関数
    ///
    /// # 引数
    ///
    /// * `program` - コマンドのプログラム名
    /// * `args` - コマンドの引数
    ///
    /// # 戻り値
    ///
    pub(crate) fn new<S1, I1, S2>(program: S1, args: I1) -> Self
    where
        S1: AsRef<OsStr>,
        I1: IntoIterator<Item = S2>,
        S2: AsRef<OsStr>,
    {
        CommandExpression {
            program: program.as_ref().to_os_string(),
            args: args
                .into_iter()
                .map(|arg| arg.as_ref().to_os_string())
                .collect_vec(),
        }
    }
    /// コマンドの実行設定を文字列に変換する関数
    ///
    /// # 戻り値
    ///
    /// コマンドの実行設定を文字列に変換した結果
    pub(crate) fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.program.to_string_lossy(),
            self.args.iter().map(|arg| arg.to_string_lossy()).join(" ")
        )
    }
}

/// コマンドの入出力のリダイレクションを格納する構造体
#[derive(Debug)]
pub(crate) struct CommandIoRedirection {
    pub stdin: Stdio,
    pub stdout: Stdio,
    pub stderr: Stdio,
}

/// コマンドの実行を行う関数
///
/// # 引数
///
/// * `expr` - コマンドの実行設定
/// * `redirect` - コマンドの入出力のリダイレクション
/// * `timeout` - コマンドの実行時間のタイムアウト
///
/// # 戻り値
///
/// コマンドの実行結果
pub async fn command_task(
    expr: CommandExpression,
    redirect: CommandIoRedirection,
    timeout: Option<f32>,
) -> CommandResult {
    if let Some(timeout_sec) = timeout {
        command_timeout(expr, redirect, timeout_sec).await
    } else {
        command(expr, redirect).await
    }
}

/// コマンドの実行を行う関数
///
/// # 引数
///
/// * `expr` - コマンドの実行設定
/// * `redirect` - コマンドの入出力のリダイレクション
///
/// # 戻り値
///
/// コマンドの実行結果
async fn command(expr: CommandExpression, redirect: CommandIoRedirection) -> CommandResult {
    let mut command = tokio::process::Command::new(&expr.program);

    let start = tokio::time::Instant::now();
    let output = command
        .args(&expr.args)
        .stdin(redirect.stdin)
        .stdout(redirect.stdout)
        .stderr(redirect.stderr)
        .spawn()
        .expect("Command failed to start")
        .wait_with_output()
        .await
        .expect("Command failed to run");
    let detail = CommandResultDetail {
        stdout: String::from_utf8_lossy(&output.stdout).into(),
        stderr: String::from_utf8_lossy(&output.stderr).into(),
        elapsed: tokio::time::Instant::now() - start,
    };
    if output.status.success() {
        CommandResult::Success(detail)
    } else {
        CommandResult::Aborted(detail)
    }
}

/// コマンドの実行を行う関数
///
/// # 引数
///
/// * `expr` - コマンドの実行設定
/// * `redirect` - コマンドの入出力のリダイレクション
/// * `timeout_sec` - コマンドの実行時間のタイムアウト
///
/// # 戻り値
///
/// コマンドの実行結果  
async fn command_timeout(
    expr: CommandExpression,
    redirect: CommandIoRedirection,
    timeout_sec: f32,
) -> CommandResult {
    let tl = Duration::from_secs_f32(timeout_sec);
    match tokio::time::timeout(tl * 2, command(expr, redirect)).await {
        Ok(wrapped_result) => match wrapped_result {
            CommandResult::Success(detail) => {
                if detail.elapsed <= tl {
                    CommandResult::Success(detail)
                } else {
                    CommandResult::Timeout(detail)
                }
            }
            CommandResult::Aborted(detail) => CommandResult::Aborted(detail),
            _ => unreachable!(),
        },
        Err(_) => CommandResult::Timeout(CommandResultDetail {
            stdout: "".into(),
            stderr: "".into(),
            elapsed: tl * 2,
        }),
    }
}

/// コマンドの実行結果を表示する関数
///
/// # 引数
///
/// * `expr` - コマンドの実行設定
///
/// # 戻り値
///
/// コマンドの実行結果
pub(crate) fn run_command_simple(expr: CommandExpression) -> CommandResult {
    log::info!("$ {}", expr.to_string());
    let result = run_task(command(
        expr,
        CommandIoRedirection {
            stdin: Stdio::piped(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        },
    ));
    describe_result(&result);
    result
}

/// コマンドの実行結果を表示する関数
///
/// # 引数
///
/// * `result` - コマンドの実行結果
///
/// # 戻り値
///
/// コマンドの実行結果
fn describe_result(result: &CommandResult) {
    match result {
        CommandResult::Success(detail) => {
            log::info!(
                "Command successfully done: {}ms elapsed.",
                detail.elapsed.as_millis()
            );
            log::debug!("stderr:\n{}", &detail.stderr);
            log::debug!("stdout:\n{}", abbr(&detail.stdout));
            log::trace!("stdout:\n{}", &detail.stdout);
        }
        CommandResult::Aborted(detail) => {
            log::error!("Command aborted: {}ms elapsed.", detail.elapsed.as_millis());
            log::error!("stderr:\n{}", &detail.stderr);
            log::debug!("stdout:\n{}", abbr(&detail.stdout));
            log::trace!("stdout:\n{}", &detail.stdout);
        }
        CommandResult::Timeout(detail) => {
            log::error!("Command timeout: {}ms elapsed.", detail.elapsed.as_millis())
        }
    }
}
