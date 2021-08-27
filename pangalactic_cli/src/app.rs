use crate::{cmdexec::Execute, opts::Command};

pub fn app_main() {
    use structopt::StructOpt;
    Command::from_args().execute();
}
