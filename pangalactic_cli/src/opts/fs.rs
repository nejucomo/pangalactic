use super::linkarg::LinkArg;
use crate::{cmd, cmdexec::Execute};
use enum_dispatch::enum_dispatch;
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[enum_dispatch(Execute)]
#[derive(Debug, StructOpt)]
#[structopt(about = "Filesystem operations")]
pub enum Fs {
    Import(FsImport),
    Export(FsExport),
    Dump(FsDump),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Import a local path into the store and print the key")]
pub struct FsImport {
    #[structopt(help = "The path to import", default_value = ".")]
    path: PathBuf,
}

impl Execute for FsImport {
    fn execute(self) -> Result<()> {
        use crate::display::display_output;

        let pglink = cmd::fs::import(&self.path)?;
        let out = pangalactic_codec::encode_string(&pglink);
        display_output(out)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Export from the store to a local path")]
pub struct FsExport {
    #[structopt(help = "The link to export")]
    link: LinkArg,

    #[structopt(help = "The path to store results")]
    path: PathBuf,
}

impl Execute for FsExport {
    fn execute(self) -> Result<()> {
        cmd::fs::export(self.link.link, &self.path)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Dump an entry to stdout: files as-is, directories as JSON")]
pub struct FsDump {
    #[structopt(help = "The link to export")]
    link: LinkArg,
}

impl Execute for FsDump {
    fn execute(self) -> Result<()> {
        cmd::fs::dump(self.link.link)
    }
}
