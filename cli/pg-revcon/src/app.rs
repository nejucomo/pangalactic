use anyhow::Result;
use pangalactic_revcon::ControlDir;
use pangalactic_runopt::{Application, RunOptions};
use pangalactic_std_store::StdStore;

use crate::options::{Command, InfoDetail, InfoOptions, InfoPathOptions, InitOptions, Options};

/// The standalone `pg-revcon` application
#[derive(Debug, Default)]
pub struct RevConApplication;

impl Application for RevConApplication {
    type Options = Options;
}

impl RunOptions<Options> for RevConApplication {
    async fn run_options(&self, options: Options) -> Result<()> {
        self.run_options(options.command).await
    }
}

impl RunOptions<Command> for RevConApplication {
    async fn run_options(&self, command: Command) -> Result<()> {
        use Command::*;

        match command {
            Info(opts) => self.run_options(opts).await,
            Init(opts) => self.run_options(opts).await,
        }
    }
}

impl RunOptions<InfoOptions> for RevConApplication {
    async fn run_options(&self, options: InfoOptions) -> Result<()> {
        self.run_options(options.detail).await
    }
}

impl RunOptions<InfoDetail> for RevConApplication {
    async fn run_options(&self, detail: InfoDetail) -> Result<()> {
        match detail {
            InfoDetail::Path(opts) => self.run_options(opts).await,
        }
    }
}

impl RunOptions<InfoPathOptions> for RevConApplication {
    async fn run_options(&self, _: InfoPathOptions) -> Result<()> {
        let ctldir = ControlDir::find_from_current_dir()?;
        println!("{ctldir}");
        Ok(())
    }
}

impl RunOptions<InitOptions> for RevConApplication {
    async fn run_options(&self, options: InitOptions) -> Result<()> {
        let mut store = StdStore::default();
        let ctldir = ControlDir::initialize(&mut store, options.workdir).await?;
        println!("{ctldir}");
        Ok(())
    }
}
