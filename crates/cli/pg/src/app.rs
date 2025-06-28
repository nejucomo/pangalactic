use anyhow::Result;
use pangalactic_revcon::Workspace;
use pangalactic_runopt::{Application, RunApp};
use pangalactic_std_store::{StdLayerInner, StdStore};
use pangalactic_store_dirdb::DirDbStore;

use crate::options::{Command, InfoDetail, InfoOptions, InfoPathOptions, InitOptions, Options};

impl Application for Options {
    async fn run(self) -> anyhow::Result<()> {
        let sli: StdLayerInner<DirDbStore> = StdStore::from(self.dirdb).into();
        self.command.run_app(sli).await
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

impl RunApp<StdLayerInner<DirDbStore>> for InfoPathOptions {
    async fn run_app(self, store: StdLayerInner<DirDbStore>) -> Result<()> {
        let ws = Workspace::find_from_current_dir(store).await?;
        println!("{ws}");
        Ok(())
    }
}

impl RunApp<StdLayerInner<DirDbStore>> for InitOptions {
    async fn run_app(self, store: StdLayerInner<DirDbStore>) -> Result<()> {
        let ctldir = Workspace::initialize(store, self.workdir).await?;
        println!("{ctldir}");
        Ok(())
    }
}
