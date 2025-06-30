use std::fmt::Debug;
use std::future::Future;

use anyhow::Result;

use crate::RunApp;

pub trait Application: Send + Debug + clap::Parser {
    fn run_main() -> impl Future<Output = Result<()>> + Send {
        Box::pin(async {
            pangalactic_log::init()?;

            let logargs = std::env::args().collect::<Vec<_>>();
            tracing::debug!(?logargs);

            let appopts = Self::parse();
            tracing::trace!(?appopts);

            appopts.run().await
        })
    }

    fn run(self) -> impl Future<Output = Result<()>> + Send;

    fn run_command<C>(self, command: C) -> impl Future<Output = Result<()>> + Send
    where
        C: RunApp<Self>,
    {
        command.run_app(self)
    }
}
