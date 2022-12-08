use include_dir::{include_dir, Dir};

static WASM_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/guests/target/wasms");

pub fn get_wasm_bytes(name: &str) -> anyhow::Result<&'static [u8]> {
    get_wasm_bytes_from_dir(&WASM_DIR, name)
}

#[cfg(test_guests)]
static TEST_WASM_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/guests/target/test-wasms");

#[cfg(test_guests)]
pub fn get_test_wasm_bytes(name: &str) -> anyhow::Result<&'static [u8]> {
    get_wasm_bytes_from_dir(&TEST_WASM_DIR, name)
}

fn get_wasm_bytes_from_dir(dir: &Dir<'static>, name: &str) -> anyhow::Result<&'static [u8]> {
    let filename = format!("{name}.wasm");
    let f = dir
        .get_file(&filename)
        .ok_or_else(|| anyhow::Error::msg(format!("not found: {filename:?}")))?;
    Ok(f.contents())
}

#[test]
fn get_wasm_bytes_for_get_derivation() -> anyhow::Result<()> {
    get_wasm_bytes("get_derivation")?;
    Ok(())
}
