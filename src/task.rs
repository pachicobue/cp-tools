use std::{fmt::Debug, future::Future};

use color_eyre::eyre::{Context, Result};
use tokio;

pub(crate) fn run_task<
    T: Debug + Clone + Send + 'static,
    F: Future<Output = Result<T>> + Send + 'static,
>(
    task: F,
) -> Result<T> {
    let results = run_tasks(vec![task])?;
    Ok(results.get(0).unwrap().clone())
}

pub(crate) fn run_tasks<
    T: Debug + Clone + Send + 'static,
    F: Future<Output = Result<T>> + Send + 'static,
>(
    tasks: Vec<F>,
) -> Result<Vec<T>> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .wrap_err("Failed to build tokio-runtime.")?;
    runtime
        .block_on(runtime.spawn(async move {
            let handles = tasks
                .into_iter()
                .map(|task| tokio::task::spawn(async move { task.await }));
            let mut results = vec![];
            for handle in handles {
                results.push(handle.await.wrap_err("Failed to join.")??)
            }
            Ok(results)
        }))
        .wrap_err("Failed to join tokio tasks.")?
}
