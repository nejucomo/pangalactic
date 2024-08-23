mod runner;
mod testdir;

use anyhow::Result;
use test_case::test_case;

use self::runner::Runner;

macro_rules! caseinfo {
    ( $x:expr ) => {
        (stringify!($x), $x)
    };
}

#[test_case(caseinfo!(setups::noop))]
fn init_idempotence<F>((name, setup): (&str, F)) -> Result<()>
where
    F: FnOnce(&Runner<'_>) -> Result<()>,
{
    let testcasedir = testdir::setup(&format!("init-idempotence_{name}"))?;
    let runner = Runner::new(&testcasedir, []);
    setup(&runner)?;

    let stdout = runner.pg(["init"], "")?.exit_ok()?;
    assert_eq!(stdout.trim(), "./.pg");

    Ok(())
}

mod setups {
    use super::{Result, Runner};

    pub fn noop<'a>(_: &Runner<'a>) -> Result<()> {
        Ok(())
    }
}
