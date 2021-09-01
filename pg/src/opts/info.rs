use crate::cmd;
use pangalactic_cli::OutputCommand;
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Display repository info")]
pub struct Info {
    #[structopt(help = "The path to query", default_value = ".")]
    path: PathBuf,
}

impl OutputCommand for Info {
    type Output = crate::repo::Repo;

    fn execute_output(&self) -> Result<Self::Output> {
        cmd::info(&self.path)
    }
}
