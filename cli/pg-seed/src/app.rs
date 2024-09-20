use anyhow::Result;
use pangalactic_manifest::FullManifest;
use pangalactic_runopt::{Application, RunOptions};
use pangalactic_seed::Seed;
use pangalactic_std_store::{StdMemStore, StdStore};

use crate::options::{Command, InstallOptions, ListOptions, Options};

/// The standalone `pg-seed` application
#[derive(Debug, Default)]
pub struct SeedApplication;

impl Application for SeedApplication {
    type Options = Options;
}

impl RunOptions<Options> for SeedApplication {
    async fn run_options(&self, options: &Options) -> Result<()> {
        self.run_options(&options.command).await
    }
}

impl RunOptions<Command> for SeedApplication {
    async fn run_options(&self, command: &Command) -> Result<()> {
        use Command::*;

        match command {
            List(opts) => self.run_options(opts).await,
            Install(opts) => self.run_options(opts).await,
        }
    }
}

impl RunOptions<ListOptions> for SeedApplication {
    async fn run_options(&self, _: &ListOptions) -> Result<()> {
        let mut store = StdMemStore::default();
        let link = store.commit(Seed).await?;
        let mani: FullManifest<_> = store.load(&link).await?;
        println!("{mani}");
        Ok(())
    }
}

impl RunOptions<InstallOptions> for SeedApplication {
    async fn run_options(&self, _: &InstallOptions) -> Result<()> {
        let mut store = StdStore::default();
        let link = store.commit(Seed).await?;
        println!("{link}");
        Ok(())
    }
}
