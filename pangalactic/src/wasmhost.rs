mod externals;
mod instance;

use wasmi::Error;

pub use self::instance::Instance;

pub fn load_module(bytes: &[u8]) -> Result<Instance, Error> {
    Instance::load_module(bytes)
}
