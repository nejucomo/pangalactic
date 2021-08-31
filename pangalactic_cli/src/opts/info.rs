use crate::{cmd, cmdexec::Execute};
use pangalactic_appdirs::AppDirs;
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Display repository info")]
pub struct Info {
    #[structopt(short, long, help = "The path to query", default_value = ".")]
    path: PathBuf,

    #[structopt(short, long, help = "Represent info in JSON")]
    json: bool,
}

impl Execute for Info {
    fn execute(self, dirs: AppDirs) -> Result<()> {
        cmd::info(dirs, &self.path, self.json)
    }
}
