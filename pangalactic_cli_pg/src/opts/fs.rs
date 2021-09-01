use super::linkarg::LinkArg;
use crate::{cmd, cmdexec::Execute};
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Import a local path into the store and print the key")]
pub struct Import {
    #[structopt(help = "The path to import", default_value = ".")]
    path: PathBuf,
}

impl Execute for Import {
    fn execute(self) -> Result<()> {
        use crate::display::display_output;

        let pglink = cmd::fs::import(&self.path)?;
        let out = pangalactic_codec::encode_string(&pglink);
        display_output(out)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Export from the store to a local path")]
pub struct Export {
    #[structopt(help = "The link to export")]
    link: LinkArg,

    #[structopt(help = "The path to store results")]
    path: PathBuf,
}

impl Execute for Export {
    fn execute(self) -> Result<()> {
        cmd::fs::export(self.link.link, &self.path)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Dump an entry to stdout: files as-is, directories as JSON")]
pub struct Dump {
    #[structopt(help = "The link to export")]
    link: LinkArg,
}

impl Execute for Dump {
    fn execute(self) -> Result<()> {
        cmd::fs::dump(self.link.link)
    }
}
