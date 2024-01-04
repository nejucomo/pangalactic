#![feature(exit_status_error)]

use anyhow_std::PathAnyhow;
use pangalactic_hash::Hash;
use std::path::{Path, PathBuf};
use test_case::test_case;

fn run_pg(testcasedir: &Path, args: &[&str], stdin: &str) -> anyhow::Result<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut cmd = Command::new(dbg!(env!("CARGO_BIN_EXE_pg")));
    cmd.args(args);
    cmd.env("XDG_DATA_HOME", testcasedir.join("datahome"));
    cmd.current_dir(testcasedir);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());

    let child = dbg!(cmd).spawn()?;

    child.stdin.as_ref().unwrap().write_all(stdin.as_bytes())?;
    let cmdout = dbg!(child.wait_with_output())?;

    cmdout.status.exit_ok()?;
    let outtext = String::from_utf8(cmdout.stdout)?;
    Ok(outtext)
}

#[test_case("")]
#[test_case("Hello World!")]
fn put_get_round_trip(input: &str) -> anyhow::Result<()> {
    dbg!(std::process::id());

    let testcasedir = get_test_case_dir(&format!(
        "put_get_round_trip-{}",
        input.replace(' ', "_").replace('!', "_")
    ))?;

    let rawlink = run_pg(&testcasedir, &["store", "put"], input)?;
    let link = rawlink.trim();

    let output = run_pg(&testcasedir, &["store", "get", link], "")?;
    assert_eq!(input, output);
    Ok(())
}

#[test_case("-", "-", "foo", "foo")]
#[test_case(
    "-",
    "pgd:",
    "fake stdin",
    "pgd:file:Cg:ylUEdJHRegaIb0wKCFE9ChOpYaetLrP3CHsHFGsk0jg"
)]
#[test_case("input/a-file", "-", "ignored", "Hello World!")]
#[test_case(
    "input/a-file",
    "pgd:",
    "ignored",
    "pgd:file:DA:XKeBWty0hOmhNsEe_mnB1TAXbVSbXRjQOOtSgLSzRww"
)]
// #[test_case("input/a-dir", "-")]
// #[test_case("input/a-dir", "pgd:")]
fn xfer(arg_in: &str, arg_out: &str, stdin: &str, stdout_expected: &str) -> anyhow::Result<()> {
    let testcasedir = get_test_case_dir(&format!(
        "xfer-{}",
        Hash::of(format!("{:?}", (arg_in, arg_out))),
    ))?;

    let inputdir = testcasedir.join("input");
    inputdir.create_dir_all_anyhow()?;
    inputdir.join("a-file").write_anyhow("Hello World!")?;

    let stdout_actual = run_pg(&testcasedir, &["store", "xfer", arg_in, arg_out], stdin)?;

    assert_eq!(stdout_actual.trim_end(), stdout_expected);
    Ok(())
}

fn get_test_case_dir(dataset: &str) -> anyhow::Result<PathBuf> {
    let tcd = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("cli_store_tests_data")
        .join(dataset);

    std::fs::remove_dir_all(&tcd).or_else(|e| {
        use std::io::ErrorKind::NotFound;

        if e.kind() == NotFound {
            // It's ok if the directory did not already exist:
            Ok(())
        } else {
            Err(e)
        }
    })?;

    tcd.create_dir_anyhow()?;
    Ok(tcd)
}
