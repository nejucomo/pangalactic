use anyhow::Result;
use pangalactic_endpoint::{Endpoint, Stdio};
use pangalactic_runopt::{Application, RunApp};
use pangalactic_std_store::StdStore;

use crate::options::{Command, GetOptions, Options, PutOptions, XferOptions};

/// The standalone `pg-seed` application
#[derive(Debug, Default)]
pub struct StoreApplication;

impl Application for StoreApplication {
    type Options = Options;
}

impl RunApp<StoreApplication> for Options {
    async fn run_app(self, app: StoreApplication) -> Result<()> {
        self.command.run_app(app).await
    }
}

impl<A> RunApp<A> for Command
where
    A: Send,
    PutOptions: RunApp<A>,
    GetOptions: RunApp<A>,
    XferOptions: RunApp<A>,
{
    async fn run_app(self, app: A) -> Result<()> {
        use Command::*;

        match self {
            Put(opts) => opts.run_app(app).await,
            Get(opts) => opts.run_app(app).await,
            Xfer(opts) => opts.run_app(app).await,
        }
    }
}

impl RunApp<StoreApplication> for PutOptions {
    async fn run_app(self, _: StoreApplication) -> Result<()> {
        let mut store = StdStore::default();
        let link = store.transfer(Stdio, ()).await?;
        println!("{link}");
        Ok(())
    }
}

impl RunApp<StoreApplication> for GetOptions {
    async fn run_app(self, _: StoreApplication) -> Result<()> {
        let mut store = StdStore::default();
        store.transfer(self.source, Stdio).await?;
        Ok(())
    }
}

impl RunApp<StoreApplication> for XferOptions {
    async fn run_app(self, _: StoreApplication) -> Result<()> {
        let XferOptions {
            excludes,
            source,
            dest,
        } = self;

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
