use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pg", about = "Pangalactic Revision Control")]
pub enum Command {
    Fs(Fs),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Filesystem operations")]
pub enum Fs {
    #[structopt(about = "Import a local path into the store and print the key")]
    Import {
        #[structopt(help = "The path to import, default: ./")]
        path: Option<PathBuf>,
    },

    #[structopt(about = "Export from the store to a local path.")]
    Export {
        #[structopt(help = "The key to export")]
        key: String, // FIXME: Use correct type.

        #[structopt(help = "The path to store results")]
        path: PathBuf,
    },
}
