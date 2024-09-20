use std::future::Future;

use anyhow::Result;
use clap::Parser;

use crate::RunOptions;

pub trait Application: Send + Default + RunOptions<Self::Options> {
    type Options: Send + std::fmt::Debug + clap::Parser;

    fn run_main() -> impl Future<Output = Result<()>> + Send {
        Box::pin(async {
            pangalactic_log::init()?;

            let logargs = std::env::args().collect::<Vec<_>>();
            tracing::debug!(?logargs);

            let opts: Self::Options = Self::Options::parse();
            tracing::trace!(?opts);

            let app = Self::default();
            app.run_options(&opts).await
        })
    }
}
