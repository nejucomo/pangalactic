use anyhow::Result;
use include_dir::{include_dir, Dir};
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_store::{Commit, Store};

use crate::log;

pub struct LibDerive;

impl<S> Commit<LinkDirectoryLayer<S>> for LibDerive
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> Result<<LinkDirectoryLayer<S> as Store>::CID> {
        let mut d = LinkDirectory::default();
        let mut testdir = LinkDirectory::default();

        for name in iter_wasm_names() {
            let bytes = get_wasm_bytes(name)?;
            let link = store.commit(bytes).await?;

            let (d, n, prefix) = if let Some(testname) = name.strip_prefix("test_") {
                (&mut testdir, testname, "libderive/test/")
            } else {
                (&mut d, name, "libderive/")
            };

            log::trace_insert(prefix, d, n, link)?;
        }

        let testlink = store.commit(testdir).await?;
        log::trace_insert("libderive/", &mut d, "test", testlink)?;

        store.commit(d).await
    }
}

static WASM_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../seed-crates/target/wasms");

pub fn iter_wasm_names() -> impl Iterator<Item = &'static str> {
    WASM_DIR.files().map(|f| {
        f.path()
            .file_name()
            .expect("filename")
            .to_str()
            .expect("utf8")
            .strip_suffix(".wasm")
            .expect(r#"".wasm" suffix"#)
    })
}

pub fn get_wasm_bytes(name: &str) -> Result<&'static [u8]> {
    let filename = format!("{name}.wasm");
    let f = WASM_DIR
        .get_file(&filename)
        .ok_or_else(|| anyhow::anyhow!("not found: {filename:?}"))?;
    Ok(f.contents())
}

#[test]
fn get_wasm_bytes_for_get_plan() -> Result<()> {
    get_wasm_bytes("get_plan")?;
    Ok(())
}
