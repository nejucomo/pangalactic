use crate::{cmdexec::Execute, opts::Command};
use pangalactic_appdirs::AppDirs;

pub fn app_main() -> std::io::Result<()> {
    use structopt::StructOpt;

    let appdirs = AppDirs::new(crate::APP_NAME)?;
    Command::from_args().execute(appdirs)
}
