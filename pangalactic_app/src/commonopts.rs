use crate::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CommonOptions {
    #[structopt(flatten)]
    logopts: pangalactic_logger::LogOptions,
}

impl Command for CommonOptions {
    fn execute(&self) -> std::io::Result<()> {
        self.logopts.init()
    }
}
