use std::future::Future;

use anyhow::Result;

pub trait RunApp<A> {
    fn run_app(self, app: A) -> impl Future<Output = Result<()>> + Send;
}

impl<T, A> RunApp<A> for Option<T>
where
    T: Default + RunApp<A>,
{
    fn run_app(self, app: A) -> impl Future<Output = Result<()>> + Send {
        self.unwrap_or_default().run_app(app)
    }
}
