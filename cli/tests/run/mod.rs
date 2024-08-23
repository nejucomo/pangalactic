use std::{path::Path, process::ExitStatus};

use anyhow::Result;

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

pub fn pg<'a, I>(testcasedir: &Path, args: I, stdin: &str) -> Result<Output>
where
    I: IntoIterator<Item = &'a str>,
{
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pg"));
    for arg in args {
        cmd.arg(arg);
    }
    cmd.env("XDG_DATA_HOME", testcasedir);
    cmd.current_dir(testcasedir);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let child = cmd.spawn()?;

    child.stdin.as_ref().unwrap().write_all(stdin.as_bytes())?;
    let cmdout = child.wait_with_output()?;
    Output::try_from(cmdout)
}
