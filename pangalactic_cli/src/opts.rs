mod linkarg;

use self::linkarg::LinkArg;
use crate::{cmd, cmdexec::Execute};
use pangalactic_appdirs::AppDirs;
use pangalactic_logger::LogOptions;
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pg", about = "Pangalactic Revision Control")]
pub struct Options {
    #[structopt(flatten)]
    pub logging: LogOptions,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Fs(Fs),
}

impl Execute for Command {
    fn execute(self, dirs: AppDirs) -> Result<()> {
        match self {
            Command::Fs(x) => x.execute(dirs),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Filesystem operations")]
pub enum Fs {
    Import(FsImport),
    Export(FsExport),
    Dump(FsDump),
}

impl Execute for Fs {
    fn execute(self, dirs: AppDirs) -> Result<()> {
        match self {
            Fs::Import(x) => x.execute(dirs),
            Fs::Export(x) => x.execute(dirs),
            Fs::Dump(x) => x.execute(dirs),
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
    fn execute(self, dirs: AppDirs) -> Result<()> {
        cmd::fs_import(dirs, &self.path.unwrap_or(PathBuf::from(".")))
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Export from the store to a local path")]
pub struct FsExport {
    #[structopt(help = "The link to export")]
    link: LinkArg,

    #[structopt(help = "The path to store results")]
    path: PathBuf,
}

impl Execute for FsExport {
    fn execute(self, dirs: AppDirs) -> Result<()> {
        cmd::fs_export(dirs, self.link.link, &self.path)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Dump an entry to stdout: files as-is, directories as JSON")]
pub struct FsDump {
    #[structopt(help = "The link to export")]
    link: LinkArg,
}

impl Execute for FsDump {
    fn execute(self, dirs: AppDirs) -> Result<()> {
        cmd::fs_dump(dirs, self.link.link)
    }
}
