mod runner;
mod testdir;

use anyhow::Result;
use pangalactic_revcon::CONTROL_DIR_NAME;
use test_case::test_case;

use self::runner::Runner;

macro_rules! caseinfo {
    ( $x:expr ) => {
        (stringify!($x), $x)
    };
}

#[test_case(caseinfo!(setups::noop))]
#[test_case(caseinfo!(setups::preseeded))]
fn init_idempotence<F>((name, setup): (&str, F)) -> Result<()>
where
    F: FnOnce(&Runner<'_>) -> Result<()>,
{
    let testcasedir = testdir::setup(&format!("init-idempotence_{name}"))?;
    let runner = Runner::new(&testcasedir, []);
    setup(&runner)?;

    let stdout = runner.pg(["init"], "")?.exit_ok()?;
    assert!(stdout.trim().ends_with(CONTROL_DIR_NAME));

    let globspath = testcasedir.join(CONTROL_DIR_NAME).join("ignore.globs");
    let globs = std::fs::read_to_string(globspath)?;
    assert_eq!(globs, ".git\n");

    Ok(())
}

mod setups {
    use super::{Result, Runner};

    pub fn noop<'a>(_: &Runner<'a>) -> Result<()> {
        Ok(())
    }

    pub fn preseeded<'a>(runner: &Runner<'a>) -> Result<()> {
        dbg!(runner
            .pg(["util", "store", "seed", "install"], "")?
            .exit_ok()?);
        Ok(())
    }
}
