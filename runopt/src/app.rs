use std::future::Future;

use anyhow::Result;
use clap::Parser;

use crate::RunApp;

pub trait Application: Send + Default {
    type Options: Send + std::fmt::Debug + clap::Parser + RunApp<Self>;

    fn run_main() -> impl Future<Output = Result<()>> + Send {
        Box::pin(async {
            pangalactic_log::init()?;

            let logargs = std::env::args().collect::<Vec<_>>();
            tracing::debug!(?logargs);

            let opts: Self::Options = Self::Options::parse();
            tracing::trace!(?opts);

            let app = Self::default();
            opts.run_app(app).await
        })
    }
}
