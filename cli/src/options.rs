use crate::dagops::{AnyPathDo, DagOps, LinkDo};
use clap::{Args, Parser, Subcommand};
use pangalactic_dir::Name;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Options {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub async fn run(self) -> anyhow::Result<()> {
        use Command::*;
        use StoreCommand::*;
        use StoreDirCommand::*;
        use StoreFileCommand::*;

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
                    Copy(opts) => dops.store_copy(opts.source, opts.dest).await,
                }
            }
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Store(StoreCommand),
}

/// Interact directly with the store
#[derive(Debug, Subcommand)]
pub enum StoreCommand {
    #[command(subcommand)]
    File(StoreFileCommand),
    #[command(subcommand)]
    Dir(StoreDirCommand),
    Copy(StoreCopyOptions),
}

/// Low-level file operations
#[derive(Debug, Subcommand)]
pub enum StoreFileCommand {
    /// Insert the file on stdin and print its key on stdout
    Put,
    /// Send the given file to stdout
    Get(StoreFileGetOptions),
}

/// Low-level dir operations
#[derive(Debug, Subcommand)]
pub enum StoreDirCommand {
    /// Print the link for the empty directory
    Empty,
    Link(StoreDirLinkOptions),
    Unlink(StoreDirUnlinkOptions),
}

/// Set a link within a directory
#[derive(Debug, Args)]
pub struct StoreDirLinkOptions {
    dir: LinkDo,
    name: Name,
    target: LinkDo,
}

/// Set a link within a directory
#[derive(Debug, Args)]
pub struct StoreDirUnlinkOptions {
    dir: LinkDo,
    name: Name,
}

#[derive(Debug, Args)]
pub struct StoreFileGetOptions {
    /// The link to get
    link: LinkDo,
}

/// Copy files or directories within or across store or host
#[derive(Debug, Args)]
pub struct StoreCopyOptions {
    /// The source path
    source: AnyPathDo,
    /// The destination path
    dest: AnyPathDo,
}
