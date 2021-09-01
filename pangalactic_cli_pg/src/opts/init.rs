use crate::cmd;
use pangalactic_cli::Command;
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Initialize a repository")]
pub struct Init {
    #[structopt(help = "The path to import", default_value = ".")]
    path: PathBuf,
}

impl Command for Init {
    fn execute(&self) -> Result<()> {
        cmd::init(&self.path)
    }
}
