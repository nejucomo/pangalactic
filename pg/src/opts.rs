mod fs;
mod info;
mod init;
mod linkarg;

use pangalactic_app::Command;
use std::io::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pg", about = "Pangalactic revision control")]
pub struct Options {
    #[structopt(flatten)]
    common: pangalactic_app::CommonOptions,

    #[structopt(subcommand)]
    cmd: Subcommand,
}

impl Command for Options {
    fn execute(&self) -> Result<()> {
        self.common.execute()?;
        self.cmd.execute()
    }
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
