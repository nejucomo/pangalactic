mod fs;
mod info;
mod init;
mod linkarg;

use crate::cmdexec::Execute;
use enum_dispatch::enum_dispatch;
use pangalactic_appdirs::AppDirs;
use pangalactic_logger::LogOptions;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pg", about = "Pangalactic Revision Control")]
pub struct Options {
    #[structopt(flatten)]
    pub logging: LogOptions,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[enum_dispatch(Execute)]
#[derive(Debug, StructOpt)]
pub enum Command {
    Info(self::info::Info),
    Init(self::init::Init),
    Fs(self::fs::Fs),
}
