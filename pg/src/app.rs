use crate::opts::Options;
use pangalactic_appdirs::AppDirs;
use std::io::Result;

pub fn app_main() -> Result<()> {
    use pangalactic_cli::Command;
    use structopt::StructOpt;

    let opts = Options::from_args(); // FIXME: handle Result;
    opts.logging.init()?;
    log::debug!("Executing: {:#?}", &opts);
    opts.cmd.execute()
}

pub fn get_appdirs() -> Result<AppDirs> {
    AppDirs::new(crate::APP_NAME)
}
