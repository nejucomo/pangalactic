use crate::{cmd, cmdexec::Execute};
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Display repository info")]
pub struct Info {
    #[structopt(help = "The path to query", default_value = ".")]
    path: PathBuf,
}

impl Execute for Info {
    fn execute(self) -> Result<()> {
        use crate::display::display_output;

        let out = cmd::info(&self.path)?;
        display_output(out)
    }
}
