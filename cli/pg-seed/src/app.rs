use anyhow::Result;
use pangalactic_manifest::FullManifest;
use pangalactic_runopt::{Application, RunApp};
use pangalactic_seed::Seed;
use pangalactic_std_store::{StdMemStore, StdStore};

use crate::options::{Command, InstallOptions, ListOptions, Options};

/// The standalone `pg-seed` application
#[derive(Debug, Default)]
pub struct SeedApplication;

impl Application for SeedApplication {
    type Options = Options;
}

impl RunApp<SeedApplication> for Options {
    async fn run_app(self, app: SeedApplication) -> Result<()> {
        self.command.run_app(app).await
    }
}

impl<A> RunApp<A> for Command
where
    A: Send,
    ListOptions: RunApp<A>,
    InstallOptions: RunApp<A>,
{
    async fn run_app(self, app: A) -> Result<()> {
        use Command::*;

        match self {
            List(opts) => opts.run_app(app).await,
            Install(opts) => opts.run_app(app).await,
        }
    }
}

impl RunApp<SeedApplication> for ListOptions {
    async fn run_app(self, _: SeedApplication) -> Result<()> {
        let mut store = StdMemStore::default();
        let link = store.commit(Seed).await?;
        let mani: FullManifest<_> = store.load(&link).await?;
        println!("{mani}");
        Ok(())
    }
}

impl RunApp<SeedApplication> for InstallOptions {
    async fn run_app(self, _: SeedApplication) -> Result<()> {
        let mut store = StdStore::default();
        let link = store.commit(Seed).await?;
        println!("{link}");
        Ok(())
    }
}
