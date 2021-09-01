use std::io::Result;

pub trait Command: Sized + structopt::StructOpt {
    fn execute(&self) -> Result<()>;
}

pub trait OutputCommand: Sized + structopt::StructOpt {
    type Output: serde::Serialize;

    fn execute_output(&self) -> Result<Self::Output>;
}

impl<T: OutputCommand> Command for T {
    fn execute(&self) -> Result<()> {
        let out = self.execute_output()?;
        crate::display::display_output(out)
    }
}
