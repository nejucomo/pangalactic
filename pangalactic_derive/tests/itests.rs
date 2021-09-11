use pangalactic_derive::derive;
use pangalactic_memstore::MemStore;
use pangalactic_nodestore::{LinkFor, NodeStore};
use std::io::Result;
use std::path::PathBuf;

macro_rules! def_test {
    ( $name:ident : $closure:expr ) => {
        #[test]
        fn $name() -> Result<()> {
            let (setup, outlink) = derive_test(stringify!($name))?;
            $closure(setup, outlink)
        }
    };
}

def_test! {
    ident: |setup: TestSetup, outlink: LinkFor<MemStore>| {
        assert_eq!(&outlink, &setup.inputlink);
        Ok(())
    }
}

def_test! {
    get_exec: |setup: TestSetup, outlink: LinkFor<MemStore>| {
        assert_eq!(&outlink, &setup.wasmlink);
        Ok(())
    }
}

def_test! {
    link_type_is_file: |_, _| Ok(())
}

fn derive_test(itestname: &str) -> Result<(TestSetup, LinkFor<MemStore>)> {
    let mut setup = TestSetup::init(itestname)?;
    let outlink = setup.derive()?;
    Ok((setup, outlink))
}

struct TestSetup {
    nodestore: NodeStore<MemStore>,
    wasmlink: LinkFor<MemStore>,
    inputlink: LinkFor<MemStore>,
}

impl TestSetup {
    fn init(itestname: &str) -> Result<TestSetup> {
        pangalactic_logger::test_init();
        log::debug!("{:?}", std::env::current_dir());

        let mut nodestore = NodeStore::from(MemStore::new());
        let wasmpath = build_itest_derivations(itestname)?;
        let wasmlink = nodestore.import_path(&wasmpath)?;
        let inputlink = nodestore.put_file(&"Hello world!")?;
        Ok(TestSetup {
            nodestore,
            wasmlink,
            inputlink,
        })
    }

    fn derive(&mut self) -> Result<LinkFor<MemStore>> {
        let link = derive(&mut self.nodestore, &self.wasmlink, &self.inputlink)?;
        Ok(link)
    }
}

fn build_itest_derivations(itestname: &str) -> Result<PathBuf> {
    use std::process::Command;

    const WASM_TARGET: &str = "wasm32-unknown-unknown";
    const TEST_DERIVATIONS: &str = "tests/derivations";

    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--target", WASM_TARGET, "--package", itestname]);
    cmd.current_dir(TEST_DERIVATIONS);

    log::debug!("Executing: {:?}", &cmd);
    let status = cmd.status()?;
    log::debug!("Status: {:?}", &status);

    if status.success() {
        Ok(PathBuf::from(TEST_DERIVATIONS)
            .join("target")
            .join(WASM_TARGET)
            .join("debug")
            .join(format!("{}.wasm", itestname)))
    } else {
        use std::io::{Error, ErrorKind::Other};

        Err(Error::new(
            Other,
            format!(
                "Failed to execute cargo build for wasm32 (code {:?})",
                status.code()
            ),
        ))
    }
}
