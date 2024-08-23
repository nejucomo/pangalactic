use std::{path::Path, process::ExitStatus};

use anyhow::Result;

#[derive(Debug)]
pub struct Runner<'a, const K: usize> {
    pub testcasedir: &'a Path,
    prefixargs: [&'a str; K],
}

impl<'a, const K: usize> Runner<'a, K> {
    pub fn new(testcasedir: &'a Path, prefixargs: [&'a str; K]) -> Self {
        Runner {
            testcasedir,
            prefixargs,
        }
    }

    pub fn pg<I>(&self, args: I, stdin: &str) -> Result<Output>
    where
        I: IntoIterator<Item = &'a str>,
    {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let mut cmd = Command::new(env!("CARGO_BIN_EXE_pg"));
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

#[derive(Debug)]
pub struct Output {
    pub status: ExitStatus,
    pub stdout: String,
}

impl TryFrom<std::process::Output> for Output {
    type Error = anyhow::Error;

    fn try_from(cmdout: std::process::Output) -> std::result::Result<Self, Self::Error> {
        println!("{}", String::from_utf8(cmdout.stderr)?);
        let status = cmdout.status;
        let stdout = String::from_utf8(cmdout.stdout)?;
        Ok(Output { status, stdout })
    }
}

impl Output {
    pub fn exit_ok(self) -> Result<String> {
        if self.status.success() {
            Ok(self.stdout)
        } else {
            anyhow::bail!("bad exit: {self:#?}");
        }
    }
}
