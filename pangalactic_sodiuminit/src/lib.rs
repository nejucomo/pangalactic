pub fn init_if_necessary() {
    SODIUM_INIT.log_attempt();
}

lazy_static::lazy_static! {
    static ref SODIUM_INIT: SodiumInit = SodiumInit::init();
}

#[derive(Debug)]
struct SodiumInit;

impl SodiumInit {
    fn init() -> SodiumInit {
        log::debug!("Initializing rust_sodium.");
        rust_sodium::init().expect("rust_sodium::init() failed.");
        SodiumInit
    }

    fn log_attempt(&self) {
        log::trace!("{:?} init_if_necssary()", self);
    }
}
