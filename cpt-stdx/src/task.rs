use std::{fmt::Debug, future::Future};

use thiserror::Error;
use tokio::{self, task::JoinError};

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Could not execute spawned task")]
    ExecFailed(JoinError),
}

pub fn run_task<T: Clone + Debug + Send + 'static, F: Future<Output = T> + Send + 'static>(
    task: F,
) -> Result<T, TaskError> {
    run_tasks(vec![task]).map(|results| results.first().unwrap().to_owned())
}

pub fn run_tasks<T: Clone + Debug + Send + 'static, F: Future<Output = T> + Send + 'static>(
    tasks: Vec<F>,
) -> Result<Vec<T>, TaskError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime
        .block_on(runtime.spawn(async move {
            let handles = tasks.into_iter().map(|task| tokio::task::spawn(task));
            let mut results = vec![];
            for handle in handles {
                results.push(handle.await.unwrap())
            }
            results
        }))
        .map_err(TaskError::ExecFailed)
}
