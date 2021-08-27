use crate::{cmdexec::Execute, opts::Options};
use pangalactic_appdirs::AppDirs;

pub fn app_main() -> std::io::Result<()> {
    use structopt::StructOpt;

    let appdirs = AppDirs::new(crate::APP_NAME)?;
    let opts = Options::from_args(); // FIXME: handle Result;
    opts.logging.init()?;
    log::debug!("Executing: {:#?}", &opts);
    opts.cmd.execute(appdirs)
}
