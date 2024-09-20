use anyhow::Result;
use pangalactic_endpoint::{Endpoint, Stdio};
use pangalactic_runopt::{Application, RunOptions};
use pangalactic_std_store::StdStore;

use crate::options::{
    StoreCommand, StoreGetOptions, StoreOptions, StorePutOptions, StoreXferOptions,
};

/// The standalone `pg-seed` application
#[derive(Debug, Default)]
pub struct StoreApplication;

impl Application for StoreApplication {
    type Options = StoreOptions;
}

impl RunOptions<StoreOptions> for StoreApplication {
    async fn run_options(&self, options: &StoreOptions) -> Result<()> {
        self.run_options(&options.command).await
    }
}

impl RunOptions<StoreCommand> for StoreApplication {
    async fn run_options(&self, command: &StoreCommand) -> Result<()> {
        use StoreCommand::*;

        match command {
            Put(opts) => self.run_options(opts).await,
            Get(opts) => self.run_options(opts).await,
            Xfer(opts) => self.run_options(opts).await,
        }
    }
}

impl RunOptions<StorePutOptions> for StoreApplication {
    async fn run_options(&self, _: &StorePutOptions) -> Result<()> {
        let mut store = StdStore::default();
        let link = store.transfer(Stdio, ()).await?;
        println!("{link}");
        Ok(())
    }
}

impl RunOptions<StoreGetOptions> for StoreApplication {
    async fn run_options(&self, options: &StoreGetOptions) -> Result<()> {
        let mut store = StdStore::default();
        store.transfer(options.source.clone(), Stdio).await?;
        Ok(())
    }
}

impl RunOptions<StoreXferOptions> for StoreApplication {
    async fn run_options(&self, options: &StoreXferOptions) -> Result<()> {
        let StoreXferOptions {
            excludes,
            source,
            dest,
        } = options.clone();

        let mut store = StdStore::default();
        let globset = excludes.into_globset()?;
        let source = globset.filter_source(source);
        let receipt = store.transfer(source, dest).await?;
        if let Endpoint::MkHos(hos) = receipt {
            println!("{hos}")
        }
        Ok(())
    }
}
