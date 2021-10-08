use pangalactic_derive::derive;
pub use pangalactic_memstore::MemStore;
pub use pangalactic_nodestore::LinkFor;
use pangalactic_nodestore::NodeStore;
use std::io::Result;
use std::path::PathBuf;

#[macro_export]
macro_rules! def_test {
    ( $name:ident : $closure:expr ) => {
        #[test]
        fn $name() -> std::io::Result<()> {
            pangalactic_logger::test_init();
            log::info!(
                "Running derivation integration test {:?}",
                stringify!($name)
            );
            let (setuplinks, outlink) = $crate::utils::derive_test(stringify!($name))?;
            $closure(setuplinks, outlink)
        }
    };
}

pub fn derive_test(itestname: &str) -> Result<(SetupLinks, LinkFor<MemStore>)> {
    let setup = TestSetup::init(itestname)?;
    setup.derive()
}

pub struct TestSetup {
    pub nodestore: NodeStore<MemStore>,
    pub links: SetupLinks,
}

pub struct SetupLinks {
    pub wasmlink: LinkFor<MemStore>,
    pub inputlink: LinkFor<MemStore>,
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
            links: SetupLinks {
                wasmlink,
                inputlink,
            },
        })
    }

    fn derive(self) -> Result<(SetupLinks, LinkFor<MemStore>)> {
        let slinks = self.links;
        let outlink = derive(self.nodestore, &slinks.wasmlink, &slinks.inputlink)?;
        Ok((slinks, outlink))
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
