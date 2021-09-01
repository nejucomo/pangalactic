use std::io::Result;

pub trait Command: Sized + structopt::StructOpt {
    fn execute(&self) -> Result<()>;

    /// Every executable should provide a thin wrapper around `Command::execute_main` so that it is
    /// accessible as a library crate for composing higher level applications.
    ///
    /// Example:
    /// ```
    /// use structopt::StructOpt;
    /// use pangalactic_cli::Command;
    ///
    /// #[derive(StructOpt)]
    /// struct App {
    ///     #[structopt(long, default_value="friend")]
    ///     name: String,
    /// }
    ///
    /// impl Command for App {
    ///     fn execute(&self) -> std::io::Result<()> {
    ///         println!("App says: Hello, {}!", self.name);
    ///         Ok(())
    ///     }
    /// }
    ///
    /// fn main() -> std::io::Result<()> {
    ///     App::execute_main()
    /// }
    /// ```
    fn execute_main() -> Result<()> {
        // TODO: Initialize logging and handle common args.
        let me = Self::from_args();
        me.execute()
    }
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
