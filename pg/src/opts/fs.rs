use super::linkarg::LinkArg;
use crate::cmd;
use pangalactic_cli::{Command, OutputCommand};
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Import a local path into the store and print the key")]
pub struct Import {
    #[structopt(help = "The path to import", default_value = ".")]
    path: PathBuf,
}

impl OutputCommand for Import {
    type Output = String;

    fn execute_output(&self) -> Result<String> {
        // TODO: Enable pglink to be directly displayed.
        let pglink = cmd::fs::import(&self.path)?;
        Ok(pangalactic_codec::encode_string(&pglink))
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

impl Command for Export {
    fn execute(&self) -> Result<()> {
        cmd::fs::export(&self.link.link, &self.path)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Dump an entry to stdout: files as-is, directories as JSON")]
pub struct Dump {
    #[structopt(help = "The link to export")]
    link: LinkArg,
}

impl Command for Dump {
    fn execute(&self) -> Result<()> {
        cmd::fs::dump(&self.link.link)
    }
}
