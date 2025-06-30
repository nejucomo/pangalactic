use anyhow::Result;
use pangalactic_endpoint::{Endpoint, Stdio};
use pangalactic_runopt::{Application, RunApp};
use pangalactic_std_store::StdStore;

use crate::options::{Command, GetOptions, Options, PutOptions, XferOptions};

impl Application for Options {
    async fn run(self) -> Result<()> {
        let store = StdStore::from(self.dirdb);
        self.command.run_app(store).await
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

impl RunApp<StdStore> for PutOptions {
    async fn run_app(self, mut app: StdStore) -> Result<()> {
        let link = app.transfer(Stdio, ()).await?;
        println!("{link}");
        Ok(())
    }
}

impl RunApp<StdStore> for GetOptions {
    async fn run_app(self, mut app: StdStore) -> Result<()> {
        app.transfer(self.source, Stdio).await?;
        Ok(())
    }
}

impl RunApp<StdStore> for XferOptions {
    async fn run_app(self, mut app: StdStore) -> Result<()> {
        let XferOptions {
            excludes,
            source,
            dest,
        } = self;

        let globset = excludes.into_globset()?;
        let source = globset.filter_source(source);
        let receipt = app.transfer(source, dest).await?;
        if let Endpoint::MkHos(hos) = receipt {
            println!("{hos}")
        }
        Ok(())
    }
}
