#![feature(exit_status_error)]

use anyhow_std::PathAnyhow;
use pangalactic_hash::Hash;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    process::{ExitStatus, Output},
};
use test_case::test_case;

#[test_case("")]
#[test_case("Hello World!")]
fn put_get_round_trip(input: &str) -> anyhow::Result<()> {
    dbg!(std::process::id());

    let testcasedir = get_test_case_dir(&format!(
        "put_get_round_trip-{}",
        input.replace(' ', "_").replace('!', "_")
    ))?;

    let link = run_pg(&testcasedir, &["store", "put"], input)?.stdout;
    let output = run_pg(&testcasedir, &["store", "get", &link], "")?.stdout;
    assert_eq!(input, output);
    Ok(())
}

#[test_case("-", "-", "foo", "foo", "")]
#[test_case(
    "-",
    "pgd:",
    "fake stdin",
    "pgd:file:Cg:ylUEdJHRegaIb0wKCFE9ChOpYaetLrP3CHsHFGsk0jg",
    ""
)]
#[test_case("input/a-file", "-", "ignored", "Hello World!", "")]
#[test_case(
    "input/a-file",
    "pgd:",
    "ignored",
    "pgd:file:DA:XKeBWty0hOmhNsEe_mnB1TAXbVSbXRjQOOtSgLSzRww",
    ""
)]
#[test_case(
    "input/a-dir",
    "-",
    "ignored",
    "",
    "Error: cannot xfer host dir \"input/a-dir\" to stdout"
)]
#[test_case(
    "input/a-dir",
    "pgd:",
    "ignored",
    "pgd:dir:Sg:1n6cmjUele4qYR53XXf3vulKUDmNEY6D3O2lAZLxEiE",
    ""
)]
#[test_case(
    "pgd:dir:Uw:xjo5Sj2bNVfP6xX6NUUqesQXh5ZCR5Evxe6SgcYlBVI",
    "output/",
    "ignored",
    "",
    ""
)]
fn xfer(
    arg_in: &str,
    arg_out: &str,
    stdin: &str,
    stdout_expected: &str,
    stderr_expected: &str,
) -> anyhow::Result<()> {
    let testcasedir = get_test_case_dir(&format!(
        "xfer-{}",
        Hash::of(format!("{:?}", (arg_in, arg_out))),
    ))?;

    // Setup:
    let inputdir = testcasedir.join("input");
    inputdir.create_dir_all_anyhow()?;
    inputdir.join("a-file").write_anyhow("Hello World!")?;
    let adir = inputdir.join("a-dir");
    adir.create_dir_anyhow()?;
    adir.join("a").write_anyhow("apple")?;
    adir.join("b").write_anyhow("banana")?;
    let rootlink = run_pg(&testcasedir, &["store", "xfer", "input/", "pgd:"], "")?.stdout;

    // Test Target:
    let xfer_res = run_pg(&testcasedir, &["store", "xfer", arg_in, arg_out], stdin)?;

    // Verification:
    assert_eq!(xfer_res.stdout, stdout_expected);
    if stderr_expected.is_empty() {
        xfer_res.require_no_error()?;
    }
    assert_eq!(xfer_res.stderr, stderr_expected);

    if arg_in == rootlink {
        assert!(recursive_quiet_diff(inputdir, testcasedir.join(arg_out))?);
    }
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

fn run_pg(testcasedir: &Path, args: &[&str], stdin: &str) -> anyhow::Result<ProcessResult> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut cmd = Command::new(dbg!(env!("CARGO_BIN_EXE_pg")));
    cmd.args(args);
    cmd.env("XDG_DATA_HOME", testcasedir.join("datahome"));
    cmd.current_dir(testcasedir);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let child = dbg!(cmd).spawn()?;

    child.stdin.as_ref().unwrap().write_all(stdin.as_bytes())?;
    let output = dbg!(child.wait_with_output())?;
    ProcessResult::try_from(output)
}

struct ProcessResult {
    status: ExitStatus,
    stdout: String,
    stderr: String,
}

impl TryFrom<Output> for ProcessResult {
    type Error = anyhow::Error;

    fn try_from(v: Output) -> Result<Self, Self::Error> {
        let status = v.status;
        let stdout = String::from_utf8(v.stdout)?.trim_end().to_string();
        let stderr = String::from_utf8(v.stderr)?.trim_end().to_string();
        Ok(ProcessResult {
            status,
            stdout,
            stderr,
        })
    }
}

impl ProcessResult {
    fn require_no_error(&self) -> anyhow::Result<()> {
        self.status.exit_ok()?;
        assert_eq!("", self.stderr);
        Ok(())
    }
}

fn recursive_quiet_diff(a: PathBuf, b: PathBuf) -> anyhow::Result<bool> {
    let mut same = true;
    let mut stack = vec![(a, b)];

    while let Some((a, b)) = stack.pop() {
        if a.is_dir() && b.is_dir() {
            for (opt_a, opt_b) in zip_eq(read_dir(&a)?, read_dir(&b)?) {
                match (opt_a, opt_b) {
                    (Some(child_a), Some(child_b)) => {
                        stack.push((child_a, child_b));
                    }
                    (Some(p), None) => {
                        same = false;
                        dbg!(p);
                    }
                    (None, Some(p)) => {
                        same = false;
                        dbg!(p);
                    }
                    (None, None) => panic!("zip_eq postcondition violation"),
                }
            }
        }
    }

    Ok(same)
}

fn read_dir(d: &Path) -> anyhow::Result<VecDeque<PathBuf>> {
    let mut v = VecDeque::default();
    for entres in d.read_dir_anyhow()? {
        let entry = entres?;
        let childpath = entry.path();
        v.push_back(childpath)
    }
    Ok(v)
}

fn zip_eq<T>(mut a: VecDeque<T>, mut b: VecDeque<T>) -> impl Iterator<Item = (Option<T>, Option<T>)>
where
    T: Ord,
{
    use std::cmp::Ordering::*;

    let mut v = VecDeque::default();

    loop {
        let selector = match (a.front(), b.front()) {
            (None, None) => {
                return v.into_iter();
            }
            (Some(_), None) => Less,
            (None, Some(_)) => Greater,
            (Some(x), Some(y)) => x.cmp(y),
        };
        let pair = match selector {
            Less => (a.pop_front(), None),
            Greater => (None, b.pop_front()),
            Equal => (a.pop_front(), b.pop_front()),
        };
        v.push_back(pair)
    }
}
