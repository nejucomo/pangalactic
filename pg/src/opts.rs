mod info;
mod init;

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
}

impl Command for Subcommand {
    fn execute(&self) -> Result<()> {
        use Subcommand::*;

        match self {
            Info(x) => x.execute(),
            Init(x) => x.execute(),
        }
    }
}
