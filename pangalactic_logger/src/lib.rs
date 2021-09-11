use log::Level;
use std::sync::Once;
use structopt::StructOpt;

static INIT: Once = Once::new();

/// Initialize logging for tests. Full applications should use `LogOptions::init`.
pub fn test_init() {
    INIT.call_once(|| {
        simple_logger::init()
            .map_err(pangalactic_errorutil::debug_to_std_io_error)
            .unwrap()
    });
}

#[derive(Debug, StructOpt)]
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
            None => {
                if std::env::var("RUST_LOG").is_ok() {
                    simple_logger::init_with_env()
                } else {
                    simple_logger::init_with_level(Level::Info)
                }
            }
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
