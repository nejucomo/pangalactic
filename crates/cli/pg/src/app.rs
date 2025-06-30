use anyhow::Result;
use pangalactic_config::Configuration as _;
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_revcon::{RevConConfig, Workspace};
use pangalactic_runopt::{Application, RunApp};
use pangalactic_std_store::{StdLayerInner, StdStore};
use pangalactic_store_dirdb::DirDbStore;

use crate::options::{Command, InfoDetail, InfoOptions, InfoPathOptions, InitOptions, Options};

#[derive(Debug)]
struct AppState {
    rcconf: RevConConfig<CidMetaLayer<DirDbStore>>,
    store: StdLayerInner<DirDbStore>,
}

impl Application for Options {
    async fn run(self) -> anyhow::Result<()> {
        let rcconf = RevConConfig::load().await?;
        let store: StdLayerInner<DirDbStore> = StdStore::from(self.dirdb).into();
        self.command.run_app(AppState { rcconf, store }).await
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

impl RunApp<AppState> for InfoPathOptions {
    async fn run_app(self, appstate: AppState) -> Result<()> {
        let AppState { rcconf, store } = appstate;
        let ws = Workspace::find_from_current_dir(rcconf, store).await?;
        println!("{ws}");
        Ok(())
    }
}

impl RunApp<AppState> for InitOptions {
    async fn run_app(self, appstate: AppState) -> Result<()> {
        let AppState { rcconf, store } = appstate;
        let ctldir = Workspace::initialize(rcconf, store, self.workdir).await?;
        println!("{ctldir}");
        Ok(())
    }
}
