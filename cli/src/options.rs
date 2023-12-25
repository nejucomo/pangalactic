pub mod store;

use crate::dagops::DagOps;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Store(self::store::Command),
}

impl Options {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub async fn run(self) -> anyhow::Result<()> {
        use self::store::Command::*;
        use self::store::DirCommand::*;
        use self::store::FileCommand::*;
        use Command::*;

        match self.command.unwrap() {
            Store(cmd) => {
                let mut dops = DagOps::default();
                match cmd {
                    File(Put) => dops.store_file_put().await,
                    File(Get(opts)) => dops.store_file_get(&opts.link).await,
                    Dir(Empty) => dops.store_dir_empty().await,
                    Dir(Link(opts)) => {
                        dops.store_dir_link(&opts.dir, &opts.name, &opts.target)
                            .await
                    }
                    Dir(Unlink(opts)) => dops.store_dir_unlink(&opts.dir, &opts.name).await,
                    Dir(List(opts)) => dops.store_dir_list(&opts.dir).await,
                    Copy(opts) => dops.store_copy(opts.source, opts.dest).await,
                }
            }
        }
    }
}
