use std::{fmt::Debug, future::Future, time::Duration};

use color_eyre::eyre::{eyre, Context, Result};
use itertools::Itertools;
use tokio;

use crate::styled;

#[derive(Debug, Clone)]
pub(crate) enum TaskResult<T: Debug + Clone> {
    Done(T),
    RuntimeError(T),
    Timeout,
}
impl<T: Debug + Clone> TaskResult<T> {
    pub(crate) fn done(&self) -> Result<T> {
        match self {
            TaskResult::Done(detail) => Ok(detail.clone()),
            _ => Err(eyre!("Command failed: {:?}", &self)),
        }
    }
}

pub(crate) fn run_single_task<
    T: Debug + Clone + Send + 'static,
    F: Future<Output = Result<TaskResult<T>>> + Send + 'static,
>(
    task: F,
    timeout_sec: Option<f32>,
) -> Result<TaskResult<T>> {
    let results = run_multi_task(vec![task], timeout_sec)?;
    Ok(results[0].clone())
}

pub(crate) fn run_multi_task<
    T: Debug + Clone + Send + 'static,
    F: Future<Output = Result<TaskResult<T>>> + Send + 'static,
>(
    tasks: Vec<F>,
    timeout_sec: Option<f32>,
) -> Result<Vec<TaskResult<T>>> {
    let timeout_sec = timeout_sec.unwrap_or(default_timeout_sec());
    let timeout = Duration::from_secs_f32(timeout_sec);

    let target_num = tasks.len();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let runtime_results = runtime
        .block_on(runtime.spawn(async move {
            let (tx, mut rx) = tokio::sync::mpsc::channel(num_cpus::get());
            // プロセス実行用タスク
            for (i, task) in tasks.into_iter().enumerate() {
                let txi = tx.clone();
                tokio::task::spawn(async move {
                    let start_time = tokio::time::Instant::now();
                    let mut result = task.await;
                    let elapsed = tokio::time::Instant::now() - start_time;
                    if elapsed > timeout {
                        result = Ok(TaskResult::Timeout);
                    }
                    txi.send((i, result)).await.unwrap();
                });
            }
            // タイムキーパー用タスク
            tokio::task::spawn(async move {
                tokio::time::sleep(timeout * 2).await;
                tx.send((target_num, Ok(TaskResult::Timeout)))
                    .await
                    .unwrap();
            });

            let mut results = (0..target_num)
                .map(|_| Ok(TaskResult::Timeout))
                .collect_vec();
            let mut collected = 0;
            while let Some((i, result)) = rx.recv().await {
                match &result {
                    Ok(TaskResult::Done(output)) => {
                        log::trace!("{}", styled!("[{}] Successfully done.\n{:?}", i, &output));
                    }
                    Ok(TaskResult::RuntimeError(output)) => {
                        log::error!(
                            "{}",
                            styled!("[{}] Runtime error occurred.\n{:?}", i, &output).red()
                        );
                    }
                    Ok(TaskResult::Timeout) => {
                        log::error!("{}", styled!("[{}] Timeout occurred.", i).red());
                        break;
                    }
                    Err(e) => {
                        log::error!(
                            "{}",
                            styled!("[{}] Internal error occurred.\n{:?}", i, e).red()
                        );
                    }
                }
                results[i] = result;
                collected += 1;
                if collected == target_num {
                    break;
                }
            }
            results
        }))
        .context("Failed to join tokio tasks.")?;

    runtime_results.into_iter().collect()
}

const fn default_timeout_sec() -> f32 {
    30.
}
