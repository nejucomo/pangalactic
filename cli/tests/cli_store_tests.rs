#![feature(exit_status_error)]

use std::path::{Path, PathBuf};

use anyhow_std::PathAnyhow;
use test_case::test_case;

#[test_case("")]
#[test_case("Hello World!")]
fn put_get_round_trip(input: &str) -> anyhow::Result<()> {
    dbg!(std::process::id());

    let testcasedir = setup_test_case_dir(&format!(
        "put_get_round_trip-{}",
        input.replace(' ', "_").replace('!', "_")
    ))?;

    let rawlink = run_pg(&testcasedir, &["store", "put"], input)?;
    let link = rawlink.trim();

    let output = run_pg(&testcasedir, &["store", "get", link], "")?;
    assert_eq!(input, output);
    Ok(())
}

#[test]
fn xfer_tests() -> anyhow::Result<()> {
    let testcasedir = setup_test_case_dir(&format!("xfer"))?;

    // Setup:
    let hostsrcroot = testcasedir.join("src");
    hostsrcroot.create_dir_anyhow()?;
    hostsrcroot.join("file.txt").write_anyhow("Hello World!")?;
    let subdir = hostsrcroot.join("subdir");
    subdir.create_dir_anyhow()?;
    subdir.join("a").write_anyhow("Hello World!")?;
    subdir.join("b").write_anyhow("Hello")?;
    subdir.join("c").write_anyhow(" World!")?;

    // # Test operations:
    // ## Test Operation A: host to host
    let srcstr = hostsrcroot.to_str_anyhow()?;
    let dstdir = testcasedir.join("dst");
    let dststr = dstdir.to_str_anyhow()?;
    run_pg_no_out(&testcasedir, &["store", "xfer", srcstr, dststr], "")?;
    check_paths_equal(&hostsrcroot, &dstdir)?;

    // ## Test Operation B: two different hosts to same store CID:
    let srccid = run_pg(&testcasedir, &["store", "xfer", srcstr, "pg:"], "")?
        .trim_end()
        .to_string();
    dbg!(&srccid);
    let dstcid = run_pg(&testcasedir, &["store", "xfer", dststr, "pg:"], "")?
        .trim_end()
        .to_string();
    dbg!(&dstcid);
    assert_eq!(srccid, dstcid);

    // ## Test Operation C: store -> host equality
    let s2h = testcasedir.join("s2h");
    run_pg_no_out(
        &testcasedir,
        &["store", "xfer", &srccid, s2h.to_str_anyhow()?],
        "",
    )?;
    check_paths_equal(&hostsrcroot, &s2h)?;

    Ok(())
}

fn setup_test_case_dir(dataset: &str) -> anyhow::Result<PathBuf> {
    let tcd = PathBuf::from(get_test_case_dir(dataset));

    tcd.remove_dir_all_anyhow().or_else(|e| {
        use std::io::ErrorKind::NotFound;

        match e.downcast_ref::<std::io::Error>() {
            Some(e) if e.kind() == NotFound => Ok(()),
            _ => Err(e),
        }
    })?;

    tcd.create_dir_anyhow()?;

    Ok(PathBuf::from(tcd))
}

fn get_test_case_dir(dataset: &str) -> String {
    format!(
        "{}/cli_store_tests_data/{dataset}",
        env!("CARGO_TARGET_TMPDIR")
    )
}

fn run_pg_no_out(testcasedir: &Path, args: &[&str], stdin: &str) -> anyhow::Result<()> {
    let output = run_pg(testcasedir, args, stdin)?;
    assert!(output.is_empty(), "unexpected: {output:?}");
    Ok(())
}

fn run_pg(testcasedir: &Path, args: &[&str], stdin: &str) -> anyhow::Result<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut cmd = Command::new(dbg!(env!("CARGO_BIN_EXE_pg")));
    cmd.args(args);
    cmd.env("XDG_DATA_HOME", testcasedir);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());

    let child = dbg!(cmd).spawn()?;

    child.stdin.as_ref().unwrap().write_all(stdin.as_bytes())?;
    let cmdout = dbg!(child.wait_with_output())?;

    cmdout.status.exit_ok()?;
    let outtext = String::from_utf8(cmdout.stdout)?;
    Ok(outtext)
}

fn check_paths_equal(src: &Path, dst: &Path) -> anyhow::Result<()> {
    use anyhow::Context;

    check_paths_equal_inner(src, dst)
        .with_context(|| format!("{:?} != {:?}", src.display(), dst.display()))
}

fn check_paths_equal_inner(src: &Path, dst: &Path) -> anyhow::Result<()> {
    #[derive(Debug, PartialEq)]
    enum Ftype {
        File,
        Dir,
    }
    use Ftype::*;

    fn file_type(p: &Path) -> anyhow::Result<Ftype> {
        if p.is_file() {
            Ok(File)
        } else if p.is_dir() {
            Ok(Dir)
        } else {
            anyhow::bail!("unknown fs type: {:?}", p.display());
        }
    }

    match (file_type(src)?, file_type(dst)?) {
        (File, File) => {
            let srcvec = src.read_anyhow()?;
            let dstvec = dst.read_anyhow()?;
            if srcvec == dstvec {
                Ok(())
            } else {
                anyhow::bail!("files not equal");
            }
        }
        (Dir, Dir) => {
            use std::collections::BTreeSet;

            let mut visitedsrc = BTreeSet::default();

            for entres in src.read_dir_anyhow()? {
                let entry = entres?;
                let suffix = entry.path().strip_prefix_anyhow(src)?.to_path_buf();
                visitedsrc.insert(suffix);
            }

            for entres in dst.read_dir_anyhow()? {
                let dstpath = entres?.path();
                let suffix = dstpath.strip_prefix_anyhow(dst)?;
                if visitedsrc.remove(suffix) {
                    let srcpath = src.join(suffix);
                    check_paths_equal_inner(&srcpath, &dstpath)?;
                } else {
                    anyhow::bail!("missing");
                }
            }

            Ok(())
        }
        (l, r) => {
            anyhow::bail!("unmatched fs types: {l:?} != {r:?}");
        }
    }
}
