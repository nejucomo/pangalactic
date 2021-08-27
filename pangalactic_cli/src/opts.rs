use crate::{cmd, cmdexec::Execute};
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pg", about = "Pangalactic Revision Control")]
pub enum Command {
    Fs(Fs),
}

impl Execute for Command {
    fn execute(self) -> Result<()> {
        match self {
            Command::Fs(x) => x.execute(),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Filesystem operations")]
pub enum Fs {
    Import(FsImport),
    Export(FsExport),
}

impl Execute for Fs {
    fn execute(self) -> Result<()> {
        match self {
            Fs::Import(x) => x.execute(),
            Fs::Export(x) => x.execute(),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Import a local path into the store and print the key")]
pub struct FsImport {
    #[structopt(help = "The path to import, default: ./")]
    path: Option<PathBuf>,
}

impl Execute for FsImport {
    fn execute(self) -> Result<()> {
        cmd::fs_import(&self.path.unwrap_or(PathBuf::from(".")))
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Export from the store to a local path.")]
pub struct FsExport {
    #[structopt(help = "The key to export")]
    key: String, // FIXME: Use correct type.

    #[structopt(help = "The path to store results")]
    path: PathBuf,
}

impl Execute for FsExport {
    fn execute(self) -> Result<()> {
        cmd::fs_export(self.key, &self.path)
    }
}
