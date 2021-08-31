use crate::{cmd, cmdexec::Execute};
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Display repository info")]
pub struct Info {
    #[structopt(short, long, help = "The path to query", default_value = ".")]
    path: PathBuf,
}

impl Execute for Info {
    fn execute(self) -> Result<()> {
        cmd::info(std::io::stdout(), &self.path)
    }
}
