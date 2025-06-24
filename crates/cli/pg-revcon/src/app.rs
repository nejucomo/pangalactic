use anyhow::Result;
use pangalactic_revcon::ControlDir;
use pangalactic_runopt::{Application, RunApp};
use pangalactic_std_store::StdStore;

use crate::options::{Command, InfoDetail, InfoOptions, InfoPathOptions, InitOptions, Options};

/// The standalone `pg-revcon` application
#[derive(Debug, Default)]
pub struct RevConApplication;

impl Application for RevConApplication {
    type Options = Options;
}

impl<A> RunApp<A> for Options
where
    A: Send,
    InfoPathOptions: RunApp<A>,
    InitOptions: RunApp<A>,
{
    async fn run_app(self, app: A) -> Result<()> {
        self.command.run_app(app).await
    }
}

impl<A> RunApp<A> for Command
where
    A: Send,
    InfoPathOptions: RunApp<A>,
    InitOptions: RunApp<A>,
{
    async fn run_app(self, app: A) -> Result<()> {
        use Command::*;

        match self {
            Info(opts) => opts.run_app(app).await,
            Init(opts) => opts.run_app(app).await,
        }
    }
}

impl<A> RunApp<A> for InfoOptions
where
    A: Send,
    InfoPathOptions: RunApp<A>,
{
    async fn run_app(self, app: A) -> Result<()> {
        self.detail.run_app(app).await
    }
}

impl<A> RunApp<A> for InfoDetail
where
    A: Send,
    InfoPathOptions: RunApp<A>,
{
    async fn run_app(self, app: A) -> Result<()> {
        match self {
            InfoDetail::Path(opts) => opts.run_app(app).await,
        }
    }
}

impl RunApp<RevConApplication> for InfoPathOptions {
    async fn run_app(self, _: RevConApplication) -> Result<()> {
        let ctldir = ControlDir::find_from_current_dir()?;
        println!("{ctldir}");
        Ok(())
    }
}

impl RunApp<RevConApplication> for InitOptions {
    async fn run_app(self, _: RevConApplication) -> Result<()> {
        let mut store = StdStore::default();
        let ctldir = ControlDir::initialize(&mut store, self.workdir).await?;
        println!("{ctldir}");
        Ok(())
    }
}
