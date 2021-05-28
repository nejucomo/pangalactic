#![deny(warnings)]
#![feature(fn_traits)]

extern crate wasmi;

mod error;
mod wasmhost;

use std::convert::AsRef;
use std::path::Path;

pub use self::error::Error;

pub fn execute_path<P: AsRef<Path>>(guest: P) -> Result<(), Error> {
    let bytes = &(read_path(guest)?)[..];
    self::wasmhost::load_and_execute_module(bytes)
}

fn read_path<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    use std::io::Read;

    let mut f = std::fs::File::open(path)?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
