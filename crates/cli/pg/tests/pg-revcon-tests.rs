use std::path::Path;

use anyhow::Result;
use pangalactic_revcon::BOOKKEEPING_DIR_NAME;
use pangalactic_test_dir as testdir;
use pangalactic_test_runner::Runner;
use test_case::test_case;

macro_rules! caseinfo {
    ( $x:expr ) => {
        (stringify!($x), $x)
    };
}

#[test_case(caseinfo!(setups::noop))]
#[test_case(caseinfo!(setups::preseeded))]
fn init_idempotence<F>((name, setup): (&str, F)) -> Result<()>
where
    F: FnOnce(&Path) -> Result<()>,
{
    let testcasedir = testdir::setup(&format!("init-idempotence_{name}"))?;
    setup(&testcasedir)?;
    let runner = Runner::new(&testcasedir, env!("CARGO_BIN_EXE_pg-revcon"), []);

    let stdout = runner.pg(["init"], "")?.exit_ok()?;
    assert!(stdout.trim().ends_with(BOOKKEEPING_DIR_NAME));

    let tomlpath = testcasedir.join(BOOKKEEPING_DIR_NAME).join("config.toml");
    let toml = std::fs::read_to_string(tomlpath)?;
    assert!(toml.contains("exclude"), "{toml:?}");

    Ok(())
}

mod setups {
    use super::{Path, Result, Runner};

    pub fn noop(_: &Path) -> Result<()> {
        Ok(())
    }

    pub fn preseeded(testcasedir: &Path) -> Result<()> {
        // Trigger warning: ugly hacks.
        use anyhow::Context;

        let rcbin = env!("CARGO_BIN_EXE_pg-revcon");
        let seedbin = format!("{}-seed", rcbin.strip_suffix("-revcon").unwrap());
        if !Path::new(&seedbin).is_file() {
            use pangalactic_test_runner::Output;
            use std::process::Command;
            dbg!("trying to build seedbin", &seedbin);
            let baseout = Command::new("cargo")
                .args(["build", "-p", "pangalactic-cli-seed"])
                .output()?;
            let output = Output::try_from(baseout)?;
            output.exit_ok()?;
        }

        Runner::new(testcasedir, &seedbin, [])
            .pg(["install"], "")
            .with_context(|| format!("pg-seed expected in {seedbin}"))?
            .exit_ok()?;
        Ok(())
    }
}
