use std::process::ExitStatus;

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
