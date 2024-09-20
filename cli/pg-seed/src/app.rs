use anyhow::Result;
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_manifest::FullManifest;
use pangalactic_runopt::{Application, RunOptions};
use pangalactic_seed::Seed;
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_store_mem::MemStore;

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
        let mut store = LinkDirectoryLayer::<MemStore>::default();
        let link = store.commit(Seed).await?;
        let mani: FullManifest<_> = store.load(&link).await?;
        println!("{mani}");
        Ok(())
    }
}

impl RunOptions<InstallOptions> for SeedApplication {
    async fn run_options(&self, _: &InstallOptions) -> Result<()> {
        let mut store: LinkDirectoryLayer<CidMetaLayer<DirDbStore>> = Default::default();
        let link = Seed.install(&mut store).await?;
        println!("{link}");
        Ok(())
    }
}
