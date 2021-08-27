use log::Level;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Logging options")]
pub struct LogOptions {
    #[structopt(long, short, help = "Debug logging")]
    debug: bool,

    #[structopt(long, short, help = "Quiet logging: only warnings/errors")]
    quiet: bool,
}

impl LogOptions {
    pub fn init(&self) -> std::io::Result<()> {
        let optlevel = self.log_level()?;
        match optlevel {
            None => simple_logger::init_with_env(),
            Some(lvl) => simple_logger::init_with_level(lvl),
        }
        .map_err(|e| {
            use std::io::{Error, ErrorKind::Other};
            Error::new(Other, format!("Failed to initialize logging: {:?}", e))
        })?;
        log::debug!("Initialized logging with optlevel {:?}", optlevel);
        Ok(())
    }

    pub fn log_level(&self) -> std::io::Result<Option<Level>> {
        match (self.debug, self.quiet) {
            (false, false) => Ok(None),
            (true, false) => Ok(Some(Level::Debug)),
            (false, true) => Ok(Some(Level::Warn)),
            (true, true) => Err({
                use std::io::{Error, ErrorKind::InvalidInput};

                Error::new(
                    InvalidInput,
                    "Options --debug and --quiet are mutually exclusive.",
                )
            }),
        }
    }
}
