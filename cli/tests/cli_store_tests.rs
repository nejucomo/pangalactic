#![feature(exit_status_error)]

use test_case::test_case;

fn run_pg(dataset: &str, args: &[&str], stdin: &str) -> anyhow::Result<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let testdatadir = env!("CARGO_TARGET_TMPDIR");
    let testcasedir = format!("{testdatadir}/cli_store_tests_data/{dataset}");

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

#[test_case("")]
#[test_case("Hello World!")]
fn put_get_round_trip(input: &str) -> anyhow::Result<()> {
    let dataset = format!(
        "put_get_round_trip-{}",
        input.replace(' ', "_").replace('!', "_")
    );

    let rawlink = run_pg(&dataset, &["store", "put"], input)?;
    let link = rawlink.trim();

    let output = run_pg(&dataset, &["store", "get", link], "")?;
    assert_eq!(input, output);
    Ok(())
}
