use std::path::Path;

use anyhow::Result;

use crate::Output;

#[derive(Debug)]
pub struct Runner<'a> {
    pub testcasedir: &'a Path,
    bin: &'a str,
    prefixargs: Vec<&'a str>,
}

impl<'a> Runner<'a> {
    pub fn new<I>(testcasedir: &'a Path, bin: &'a str, prefixargs: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        Runner {
            testcasedir,
            bin,
            prefixargs: prefixargs.into_iter().collect(),
        }
    }

    pub fn pg<I>(&self, args: I, stdin: &str) -> Result<Output>
    where
        I: IntoIterator<Item = &'a str>,
    {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let mut cmd = Command::new(self.bin);
        for arg in self.prefixargs.clone().into_iter().chain(args) {
            cmd.arg(arg);
        }
        cmd.env("XDG_DATA_HOME", self.testcasedir);
        cmd.current_dir(self.testcasedir);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn()?;

        child.stdin.as_ref().unwrap().write_all(stdin.as_bytes())?;
        let cmdout = child.wait_with_output()?;
        Output::try_from(cmdout)
    }
}
