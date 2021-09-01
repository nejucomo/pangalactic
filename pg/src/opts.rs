mod fs;
mod info;
mod init;
mod linkarg;

use pangalactic_cli::Command;
use pangalactic_logger::LogOptions;
use std::io::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pg", about = "Pangalactic Revision Control")]
pub struct Options {
    #[structopt(flatten)]
    pub logging: LogOptions,

    #[structopt(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Info(self::info::Info),
    Init(self::init::Init),
    Fs(Fs),
}

impl Command for Subcommand {
    fn execute(&self) -> Result<()> {
        use Subcommand::*;

        match self {
            Info(x) => x.execute(),
            Init(x) => x.execute(),
            Fs(x) => x.execute(),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Filesystem operations")]
pub enum Fs {
    Import(self::fs::Import),
    Export(self::fs::Export),
    Dump(self::fs::Dump),
}

impl Command for Fs {
    fn execute(&self) -> Result<()> {
        use Fs::*;

        match self {
            Import(x) => x.execute(),
            Export(x) => x.execute(),
            Dump(x) => x.execute(),
        }
    }
}
