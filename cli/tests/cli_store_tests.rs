#![feature(exit_status_error)]

use test_case::test_case;

fn run_pg(testcasedir: &str, args: &[&str], stdin: &str) -> anyhow::Result<String> {
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

#[test_case("")]
#[test_case("Hello World!")]
fn put_get_round_trip(input: &str) -> anyhow::Result<()> {
    dbg!(std::process::id());

    let testcasedir = get_test_case_dir(&format!(
        "put_get_round_trip-{}",
        input.replace(' ', "_").replace('!', "_")
    ));

    std::fs::remove_dir_all(&testcasedir).or_else(|e| {
        use std::io::ErrorKind::NotFound;

        if e.kind() == NotFound {
            // It's ok if the directory did not already exist:
            Ok(())
        } else {
            Err(e)
        }
    })?;

    let rawlink = run_pg(&testcasedir, &["store", "put"], input)?;
    let link = rawlink.trim();

    let output = run_pg(&testcasedir, &["store", "get", link], "")?;
    assert_eq!(input, output);
    Ok(())
}

fn get_test_case_dir(dataset: &str) -> String {
    format!(
        "{}/cli_store_tests_data/{dataset}",
        env!("CARGO_TARGET_TMPDIR")
    )
}
