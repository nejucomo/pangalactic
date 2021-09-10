use pangalactic_derive::derive;
use pangalactic_memstore::MemStore;
use pangalactic_nodestore::{LinkFor, NodeStore};
use std::io::Result;
use std::path::PathBuf;

struct TestSetup {
    nodestore: NodeStore<MemStore>,
    wasmlink: LinkFor<MemStore>,
    inputlink: LinkFor<MemStore>,
}

impl TestSetup {
    fn init(itestname: &str) -> Result<TestSetup> {
        pangalactic_logger::simple_init()?;
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

#[test]
fn test_ident() -> Result<()> {
    let mut setup = TestSetup::init("ident")?;
    let outlink = setup.derive()?;
    assert_eq!(&outlink, &setup.inputlink);
    Ok(())
}

fn build_itest_derivations(itestname: &str) -> Result<PathBuf> {
    use std::process::Command;

    const WASM_TARGET: &str = "wasm32-unknown-unknown";
    const TEST_DERIVATIONS: &str = "test-derivations";

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
