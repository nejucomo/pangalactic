use crate::{cmdexec::Execute, opts::Command};

pub fn app_main() -> std::io::Result<()> {
    use structopt::StructOpt;
    Command::from_args().execute()
}
