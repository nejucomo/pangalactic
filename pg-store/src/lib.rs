mod cmd;
mod linkarg;

use crate::linkarg::LinkArg;
use pangalactic_app::{Command, OutputCommand};
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
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
#[structopt(name = "pg-store", about = "Pangalactic filesystem operations")]
pub enum Subcommand {
    Import(Import),
    Export(Export),
    Dump(Dump),
}

impl Command for Subcommand {
    fn execute(&self) -> Result<()> {
        use Subcommand::*;

        match self {
            Import(x) => x.execute(),
            Export(x) => x.execute(),
            Dump(x) => x.execute(),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Import a local path into the store and print the key")]
pub struct Import {
    #[structopt(help = "The path to import", default_value = ".")]
    path: PathBuf,
}

impl OutputCommand for Import {
    type Output = String;

    fn execute_output(&self) -> Result<String> {
        // TODO: Enable link to be directly displayed.
        let link = cmd::import(&self.path)?;
        Ok(pangalactic_codec::encode_string(&link))
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
        cmd::export(&self.link.link, &self.path)
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
        cmd::dump(&self.link.link)
    }
}
