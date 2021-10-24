use std::io::{Error, ErrorKind::Other, Result};
use std::process::Command;

fn main() -> Result<()> {
    let status = Command::new("mdbook")
        .args(&["build", "./guide"])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::new(Other, format!("{:?}", status)))
    }
}
