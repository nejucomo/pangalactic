use include_dir::{include_dir, Dir};

static WASM_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/guests/target/wasms");

pub fn get_wasm_bytes(name: &str) -> anyhow::Result<&'static [u8]> {
    let filename = format!("{name}.wasm");
    let f = WASM_DIR
        .get_file(&filename)
        .ok_or_else(|| anyhow::Error::msg(format!("not found: {filename:?}")))?;
    Ok(f.contents())
}

#[test]
fn get_wasm_bytes_for_get_derivation() -> anyhow::Result<()> {
    get_wasm_bytes("get_derivation")?;
    Ok(())
}
