use std::future::Future;

use anyhow::Result;

pub trait RunOptions<T> {
    fn run_options(&self, options: T) -> impl Future<Output = Result<()>> + Send;
}

impl<S, T> RunOptions<Option<T>> for S
where
    S: RunOptions<T>,
    T: Default + Send,
{
    fn run_options(&self, options: Option<T>) -> impl Future<Output = Result<()>> + Send {
        self.run_options(options.unwrap_or_default())
    }
}
