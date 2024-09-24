use anyhow::Result;
use pangalactic_cli_derive::options::DeriveOptions;
use pangalactic_cli_revcon::options::{
    InfoDetail, InfoOptions, InfoPathOptions, InitOptions, RevConCommand,
};
use pangalactic_cli_store::options::StoreCommand;
use pangalactic_runopt::{Application, RunOptions};

use crate::options::{PgCommand, PgOptions, UtilCommand};

/// pangalactic deterministic revision control
#[derive(Debug, Default)]
pub struct PgApplication;

impl Application for PgApplication {
    type Options = PgOptions;
}

impl RunOptions<PgOptions> for PgApplication {
    async fn run_options(&self, options: PgOptions) -> Result<()> {
        self.run_options(options.command.unwrap_or_default()).await
    }
}

impl RunOptions<PgCommand> for PgApplication {
    async fn run_options(&self, options: PgCommand) -> Result<()> {
        use PgCommand::*;

        match options {
            RevCon(opts) => self.run_options(opts).await,
            Util(opts) => self.run_options(opts).await,
        }
    }
}

impl RunOptions<RevConCommand> for PgApplication {
    async fn run_options(&self, command: RevConCommand) -> Result<()> {
        use RevConCommand::*;

        match command {
            Info(opts) => self.run_options(opts).await,
            Init(opts) => self.run_options(opts).await,
        }
    }
}

impl RunOptions<InfoOptions> for PgApplication {
    async fn run_options(&self, options: InfoOptions) -> Result<()> {
        self.run_options(options.detail.unwrap_or_default()).await
    }
}

impl RunOptions<InfoDetail> for PgApplication {
    async fn run_options(&self, detail: InfoDetail) -> Result<()> {
        match detail {
            InfoDetail::Path(opts) => self.run_options(opts).await,
        }
    }
}

impl RunOptions<InfoPathOptions> for PgApplication {
    async fn run_options(&self, _: InfoPathOptions) -> Result<()> {
        todo!()
    }
}

impl RunOptions<InitOptions> for PgApplication {
    async fn run_options(&self, _: InitOptions) -> Result<()> {
        todo!()
    }
}

impl RunOptions<UtilCommand> for PgApplication {
    async fn run_options(&self, command: UtilCommand) -> Result<()> {
        use UtilCommand::*;

        match command {
            RevCon(opts) => self.run_options(opts).await,
            Store(opts) => self.run_options(opts).await,
            Derive(opts) => self.run_options(opts).await,
        }
    }
}

impl RunOptions<StoreCommand> for PgApplication {
    async fn run_options(&self, _: StoreCommand) -> Result<()> {
        todo!()
    }
}

impl RunOptions<DeriveOptions> for PgApplication {
    async fn run_options(&self, _: DeriveOptions) -> Result<()> {
        todo!()
    }
}
