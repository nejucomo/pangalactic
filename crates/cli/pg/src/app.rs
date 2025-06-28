use std::os::unix::process::CommandExt;
use std::process::Command;

use anyhow::Result;
use pangalactic_runopt::{Application, RunApp};

use crate::{
    intosubargs::IntoSubArgs,
    options::{PgCommand, PgOptions, UtilCommand},
};

impl PgOptions {
    async fn run_subcommand<A>(self, bin: &str, args: A) -> Result<()>
    where
        A: IntoSubArgs,
    {
        let error = Command::new(bin).args(args.into_args()).exec();
        Err(anyhow::Error::from(error))
    }
}

impl Application for PgOptions {
    async fn run(self) -> Result<()> {
        todo!()
    }
}

impl RunApp<PgOptions> for PgOptions {
    async fn run_app(self, app: PgOptions) -> Result<()> {
        self.command.run_app(app).await
    }
}

impl RunApp<PgOptions> for PgCommand {
    async fn run_app(self, app: PgOptions) -> Result<()> {
        use PgCommand::*;

        match self {
            RevCon(opts) => app.run_subcommand("pg-revcon", opts).await,
            Util(opts) => opts.run_app(app).await,
        }
    }
}

impl RunApp<PgOptions> for UtilCommand {
    async fn run_app(self, app: PgOptions) -> Result<()> {
        use UtilCommand::*;

        match self {
            RevCon(opts) => app.run_subcommand("pg-revcon", opts).await,
            Store(opts) => app.run_subcommand("pg-store", opts).await,
            Derive(opts) => app.run_subcommand("pg-derive", opts).await,
        }
    }
}
