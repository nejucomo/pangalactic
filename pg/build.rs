use std::io::{Error, ErrorKind::Other, Result};
use std::process::Command;

fn main() -> Result<()> {
    let bookroot = "./guide";
    println!("cargo:rerun-if-changed={}", bookroot);
    let status = Command::new("mdbook").args(&["build", bookroot]).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::new(Other, format!("{:?}", status)))
    }
}
