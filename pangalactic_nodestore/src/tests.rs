use std::path::Path;

#[test]
fn import_export() -> std::io::Result<()> {
    use crate::NodeStore;
    use pangalactic_codecpath::CodecPath;
    use pangalactic_memstore::MemStore;
    use testdir::testdir;

    pangalactic_logger::simple_init()?;

    let td = testdir!();
    let indir = td.join("input");
    let outdir = td.join("output");

    macro_rules! fs_make {
        ( $subpath:expr => $contents:expr ) => {
            let p = indir.join($subpath);
            std::fs::create_dir_all(p.parent().unwrap())?;
            p.write_bytes($contents)?;
        };

        ( $subpath:expr ) => {
            std::fs::create_dir_all(indir.join($subpath))?;
        };
    }

    fs_make!("./greeting.txt" => b"Hello World");
    fs_make!("./foobar/foo.txt" => b"FOO!");
    fs_make!("./foobar/bar.txt" => b"BAR!");
    fs_make!("./an/empty/leaf/dir/");

    let mut store = NodeStore::from(MemStore::new());
    let link = store.import_path(&indir)?;
    store.export_path(&link, &outdir)?;

    // Check that the two directory structures are identical:
    check_paths_equal(&indir, &outdir)
}

fn check_paths_equal(a: &Path, b: &Path) -> std::io::Result<()> {
    use pangalactic_codecpath::CodecPath;

    assert!(a.exists());
    assert!(b.exists());

    if a.is_dir() {
        assert!(b.is_dir());

        for entryres in a.read_dir()? {
            let suba = entryres?.path();
            let subb = b.join(suba.file_name().unwrap());
            check_paths_equal(&suba, &subb)?;
        }
    } else {
        assert!(a.is_file());
        assert!(b.is_file());

        let bytesa = a.read_bytes()?;
        let bytesb = b.read_bytes()?;
        assert_eq!(bytesa, bytesb);
    }

    Ok(())
}
