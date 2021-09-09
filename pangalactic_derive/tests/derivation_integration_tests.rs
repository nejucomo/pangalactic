use pangalactic_derive::derive;
use pangalactic_memstore::MemStore;
use pangalactic_node::Link;
use pangalactic_nodestore::NodeStore;
use pangalactic_store::Store;
use std::io::Result;
use std::path::PathBuf;

type MemNodeStore = NodeStore<MemStore>;
type MemNodeLink = Link<<MemStore as Store>::Key>;

struct TestSetup {
    nodestore: MemNodeStore,
    wasmlink: MemNodeLink,
    inputlink: MemNodeLink,
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

    fn derive(&mut self) -> Result<MemNodeLink> {
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
