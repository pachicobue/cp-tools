use std::{fmt::Debug, future::Future};

use tokio;

pub(crate) fn run_task<
    T: Clone + Debug + Send + 'static,
    F: Future<Output = T> + Send + 'static,
>(
    task: F,
) -> T {
    run_tasks(vec![task]).first().unwrap().clone()
}

pub(crate) fn run_tasks<
    T: Clone + Debug + Send + 'static,
    F: Future<Output = T> + Send + 'static,
>(
    tasks: Vec<F>,
) -> Vec<T> {
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
        .unwrap()
}
