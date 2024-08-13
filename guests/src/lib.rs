use include_dir::{include_dir, Dir};

static WASM_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/guests/target/wasms");

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

pub fn get_wasm_bytes(name: &str) -> anyhow::Result<&'static [u8]> {
    let filename = format!("{name}.wasm");
    let f = WASM_DIR
        .get_file(&filename)
        .ok_or_else(|| anyhow::Error::msg(format!("not found: {filename:?}")))?;
    Ok(f.contents())
}

#[test]
fn get_wasm_bytes_for_get_plan() -> anyhow::Result<()> {
    get_wasm_bytes("get_plan")?;
    Ok(())
}
