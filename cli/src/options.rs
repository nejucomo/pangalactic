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
        use self::store::dir::Command::*;
        use self::store::file::Command::*;
        use self::store::tree::Command::*;
        use self::store::Command::*;
        use Command::*;

        let mut dops = DagOps::default();
        match self.command.unwrap() {
            Store(File(Put(opts))) => dops.store_file_put(opts.dest).await,
            Store(File(Get(opts))) => dops.store_file_get(&opts.link).await,
            Store(Dir(Empty)) => dops.store_dir_empty().await,
            Store(Dir(Link(opts))) => {
                dops.store_dir_link(&opts.dir, &opts.name, &opts.target)
                    .await
            }
            Store(Dir(Unlink(opts))) => dops.store_dir_unlink(&opts.dir, &opts.name).await,
            Store(Dir(List(opts))) => dops.store_dir_list(&opts.dir).await,
            Store(Tree(Manifest(opts))) => dops.store_tree_manifest(&opts.root).await,
            Store(Tree(Import(opts))) => dops.store_tree_import(&opts.src).await,
            Store(Tree(Export(opts))) => dops.store_tree_export(&opts.root, &opts.dest).await,
            Store(Copy(opts)) => dops.store_copy(opts.source, opts.dest).await,
        }
    }
}
