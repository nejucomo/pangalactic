#![deny(warnings)]
#![feature(fn_traits)]

extern crate wasmi;

mod error;
mod wasmhost;

use std::convert::AsRef;
use std::path::Path;

pub use self::error::Error;

pub fn execute_path<P: AsRef<Path>>(guest: P) -> Result<(), Error> {
    let bytes = read_path(guest)?;
    execute_module_bytes(&bytes)?;
    Ok(())
}

fn execute_module_bytes(bytes: &[u8]) -> Result<(), Error> {
    let mut host = wasmhost::load_module(bytes).unwrap();

    let guestresult = host.invoke_export("main", &[])?;

    assert_eq!(guestresult, None);
    Ok(())
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
