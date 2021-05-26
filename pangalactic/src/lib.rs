extern crate wasmi;

mod hostexternals;

use std::path::Path;
use std::convert::AsRef;


pub fn execute_path<P: AsRef<Path>>(guest: P) {
    let bytes = read_path(guest);
    execute_module_bytes(&bytes);
}

fn execute_module_bytes(bytes: &[u8]) {
    use ::wasmi::{Module, ModuleInstance, ImportsBuilder};

    let module = Module::from_buffer(bytes).unwrap();
    let instance =
        ModuleInstance::new(
            &module,
            &ImportsBuilder::default(),
        )
        .unwrap()
        .assert_no_start();

    let guestresult =
        instance.invoke_export(
            "main",
            &[],
            &mut (hostexternals::HostExternals {}),
        )
        .unwrap();

    assert_eq!(guestresult, None);
}

fn read_path<P: AsRef<Path>>(path: P) -> Vec<u8> {
    use std::io::Read;

    let mut f = std::fs::File::open(path).unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();
    buf
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
